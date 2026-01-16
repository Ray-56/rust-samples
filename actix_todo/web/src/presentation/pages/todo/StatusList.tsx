import { SortableContext } from "@dnd-kit/sortable";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Todo from "@/domains/todo/entities/Todo";
import Droppable from "./Droppable";
import TodoItem from "./TodoItem";
import { FormValues } from "./Editor";

interface StatusListProps {
  status: string;
  description?: React.ReactNode;
  todos: Todo[];
  onUpadate: (id: number, data: FormValues) => void;
  onDelete: (id: number) => void;
}

const StatusList: React.FC<StatusListProps> = (props) => {
  const { description, status, todos, onUpadate, onDelete } = props;
  return (
    <Card className="flex-1">
      <CardHeader>
        <CardTitle>{status}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent className="bg-gray-100">
        {todos.length === 0 ? (
          <p className="text-gray-500">No tasks</p>
        ) : (
          <SortableContext items={todos.map((x) => x.getId())}>
            <Droppable id="pending">
              {todos.map((todo) => (
                <TodoItem
                  key={todo.getId()}
                  todo={todo}
                  onUpdate={(vals) => onUpadate(todo.getId(), vals)}
                  onDelete={onDelete.bind(undefined, todo.getId())}
                />
              ))}
            </Droppable>
          </SortableContext>
        )}
      </CardContent>
    </Card>
  );
};

export default StatusList;
