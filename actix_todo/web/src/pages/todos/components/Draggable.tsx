import { UniqueIdentifier } from "@dnd-kit/core";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { cx } from "class-variance-authority";

interface DraggableProps {
  children: React.ReactNode;
  id: UniqueIdentifier;
  className?: string;
  onClick?: (e: React.MouseEvent) => void;
}

export const Draggable: React.FC<DraggableProps> = (props) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id: props.id,
  });
  const style = {
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <div
      ref={setNodeRef}
      className={cx(props.className, { "opacity-60": isDragging })}
      style={style}
      {...listeners}
      {...attributes}
    >
      {props.children}
    </div>
  );
};
