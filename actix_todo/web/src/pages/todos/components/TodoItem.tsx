import { GripVertical, PencilLine, Trash2 } from "lucide-react";
import { UniqueIdentifier } from "@dnd-kit/core";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cx } from "class-variance-authority";
import Todo from "@/common/domains/todo-domain/entities/todo";
import { Button } from "@/components/ui/button";
import { Editor, type FormValues } from "./Editor";

interface TodoItemProps {
  id: UniqueIdentifier;
  data?: Todo;
  className?: string;

  onUpdate?: (values: FormValues) => void;
  onDelete?: VoidFunction;
}

export const TodoItem: React.FC<TodoItemProps> = (props) => {
  const { onUpdate, onDelete, data, id, className } = props;
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <div
      ref={setNodeRef}
      className={cx(
        "p-2 flex items-center gap-x-2 rounded bg-white transform origin-[0_0] touch-manipulation",
        className,
        {
          "opacity-60": isDragging,
        }
      )}
      style={style}
    >
      <Button
        size="icon"
        variant="ghost"
        className="cursor-move"
        {...listeners}
        {...attributes}
      >
        <GripVertical />
      </Button>
      #{data?.getId()}
      <p className="flex-1">{data?.getDescription()}</p>
      <Editor
        defaultValues={{
          description: data?.getDescription() ?? "",
          status: data?.getStatus()!,
        }}
        trigger={<PencilLine className="todo-action text-primary" />}
        onSubmit={onUpdate}
      />
      <Trash2 className="todo-action text-destructive" onClick={onDelete} />
    </div>
  );
};
