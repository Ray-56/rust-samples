import { GripVertical, PencilLine, Trash2 } from "lucide-react";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cx } from "class-variance-authority";

import Todo from "@/domains/todo/entities/Todo";
import { Button } from "@/components/ui/button";
import { Editor, type FormValues } from "./Editor";

interface TodoItemProps {
  todo: Todo;
  className?: string;

  onUpdate?: (values: FormValues) => void;
  onDelete?: VoidFunction;
}

const TodoItem: React.FC<TodoItemProps> = (props) => {
  const { onUpdate, onDelete, todo, className } = props;
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: todo?.getId() });

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
      #{todo?.getId()}
      <p className="flex-1">{todo.getDescription().getValue()}</p>
      <Editor
        defaultValues={{
          description: todo.getDescription().getValue(),
          status: todo.getStatus().getValue(),
        }}
        trigger={<PencilLine className="todo-action text-primary" />}
        onSubmit={onUpdate}
      />
      <Trash2 className="todo-action text-destructive" onClick={onDelete} />
    </div>
  );
};

export default TodoItem;