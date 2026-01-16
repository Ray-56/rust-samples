import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import * as z from "zod";

import { Button } from "@/presentation/components/ui/button";
import { Input } from "@/presentation/components/ui/input";
import { Alert } from "@/presentation/components/ui/alert";

const schema = z.object({
  username: z
    .string()
    .min(3, "Username must be at least 3 characters")
    .max(50, "Username must be at most 50 characters"),
});

interface ForgotPasswordFormProps {
  onSubmit: (data: { username: string }) => Promise<void>;
  error?: string | null;
  success?: string | null;
}

export function ForgotPasswordForm({
  onSubmit,
  error,
  success,
}: ForgotPasswordFormProps) {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm({
    resolver: zodResolver(schema),
  });

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {error && <Alert variant="destructive">{error}</Alert>}
      {success && <Alert variant="default">{success}</Alert>}
      <div>
        <Input
          type="text"
          placeholder="Username"
          {...register("username")}
          className={errors.username ? "border-red-500" : ""}
        />
        {errors.username && (
          <p className="text-red-500 text-sm">{errors.username.message}</p>
        )}
      </div>
      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? "Sending..." : "Send Reset Password Link"}
      </Button>
    </form>
  );
}
