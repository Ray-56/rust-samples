import { UniqueIdentifier, useDroppable } from "@dnd-kit/core";
import { cx } from "class-variance-authority";

interface DroppableProps {
  children: React.ReactNode;
  id: UniqueIdentifier;
}

const Droppable: React.FC<DroppableProps> = (props) => {
  const { isOver, setNodeRef } = useDroppable({
    id: props.id,
  });

  return (
    <div
      ref={setNodeRef}
      className={cx("border flex flex-col gap-y-2", {
        "border-green-400": isOver,
      })}
    >
      {props.children}
    </div>
  );
};

export default Droppable;