import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { FC, useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Loader2 } from "lucide-react";

const formSchema = z.object({
  description: z.string().min(2, {
    message: "Description must be at least 2 characters.",
  }),
  status: z.enum(["pending", "doing", "completed"], {
    message: 'Status must be one of "pending", "doing", "completed".',
  }),
});

export type FormValues = z.infer<typeof formSchema>;

interface EditorProps {
  trigger?: React.ReactNode;
  defaultValues?: FormValues;
  onSubmit?: (values: FormValues) => void | Promise<void>;
}
export const Editor: FC<EditorProps> = (props) => {
  const { trigger, defaultValues, onSubmit } = props;
  const [open, setOpen] = useState(false);
  const [saveLoading, setSaveLoading] = useState(false);

  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues,
  });

  const handleSubmit = form.handleSubmit(async (values) => {
    const rst = onSubmit?.(values);
    if (rst instanceof Promise) {
      setSaveLoading(true);
      await rst.finally(() => {
        setSaveLoading(false);
      });
    }
    setOpen(false);
  });

  return (
    <Dialog
      open={open}
      onOpenChange={(open) => {
        setOpen(open);
        if (open) {
          form.reset();
        }
      }}
    >
      <DialogTrigger asChild>{trigger}</DialogTrigger>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Edit profile</DialogTitle>
          <DialogDescription>
            Make changes to your profile here. Click save when you're done.
          </DialogDescription>
        </DialogHeader>
        <Form {...form}>
          <form id="todo-editor" onSubmit={handleSubmit} className="space-y-8">
            <FormField
              control={form.control}
              name="description"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Description</FormLabel>
                  <FormControl>
                    <Input placeholder="describe what to do..." {...field} />
                  </FormControl>
                  <FormDescription>
                    This is your public display name.
                  </FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="status"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Status</FormLabel>
                  <FormControl>
                    <RadioGroup
                      defaultValue={field.value}
                      onValueChange={field.onChange}
                    >
                      <div className="flex items-center space-x-2">
                        <RadioGroupItem value="pending" id="r1" />
                        <Label htmlFor="r1">Pending</Label>
                      </div>
                      <div className="flex items-center space-x-2">
                        <RadioGroupItem value="doing" id="r2" />
                        <Label htmlFor="r2">Doing</Label>
                      </div>
                      <div className="flex items-center space-x-2">
                        <RadioGroupItem value="completed" id="r3" />
                        <Label htmlFor="r3">Completed</Label>
                      </div>
                    </RadioGroup>
                  </FormControl>
                  <FormDescription>This is your todo status.</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </form>
        </Form>
        <DialogFooter>
          <Button disabled={saveLoading} type="submit" form="todo-editor">
            {saveLoading && <Loader2 className="animate-spin" />} Save changes
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
