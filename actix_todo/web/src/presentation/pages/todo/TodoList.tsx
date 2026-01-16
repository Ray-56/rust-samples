import { useState } from "react";
import {
  DndContext,
  DragEndEvent,
  DragOverlay,
  MeasuringStrategy,
  UniqueIdentifier,
} from "@dnd-kit/core";
import * as R from "ramda";

import "./index.css";
import { Button } from "@/components/ui/button";
import { useTodo } from "@/application/todo/useTodo";
import { Alert } from "@/presentation/components/ui/alert";
import TodoService from "@/domains/todo/services/TodoService";
import Todo from "@/domains/todo/entities/Todo";
import { FormValues } from "./Editor";
import Header from "./Header";
import TodoItem from "./TodoItem";
import StatusList from "./StatusList";

export const reorderByIds = <T extends { getId: () => number }>(
  items: T[],
  ids: number[]
) => {
  const idmap = new Map<number, T>();
  items.forEach((item) => idmap.set(item.getId(), R.clone(item)));

  return ids.map((id) => idmap.get(id)!);
};

export default function TodoListPage() {
  const { todos, createTodo, updateTodo, deleteTodo, isLoading, error } =
    useTodo();

  const [activeId, setActiveId] = useState<UniqueIdentifier | null>(null);
  const [draggingItem, setDraggingItem] = useState<Todo | null>(null);

  const pendingTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "pending"
  );
  const doingTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "doing"
  );
  const completedTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "completed"
  );

  const handleAdd = async (description: string = "todo...") => {};

  const handleUpdate = async (id: number, newVals: FormValues) => {
    const todo = TodoService.getTodoById(id, todos);
  };

  const handleDelete = async (id: number) => {
    const todo = TodoService.getTodoById(id, todos);
  };

  async function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!active || !over) return;
  }

  return (
    <section className="p-4">
      <Button
        onClick={async () => {
          // reorderTodo("pending").then(console.log);
        }}
      >
        Reorder Pending Todos
      </Button>
      <Header onFinish={handleAdd} />
      {error && <Alert variant="destructive">{error}</Alert>}
      {isLoading ? (
        <p>Loading...</p>
      ) : (
        <DndContext
          measuring={{ droppable: { strategy: MeasuringStrategy.Always } }}
          onDragStart={(event) => {
            setActiveId(event.active.id);
            const item = TodoService.getTodoById(
              event.active.id as number,
              todos
            );
            setDraggingItem(item || null);
          }}
          onDragOver={(event) => {
            console.log("over", event);
          }}
          onDragEnd={handleDragEnd}
        >
          <div className="mt-4 grid gap-3 grid-cols-[repeat(auto-fill,minmax(350px,1fr))]">
            <StatusList
              status="Pending"
              description="This task is pending and not started yet."
              todos={pendingTodos}
              onUpadate={handleUpdate}
              onDelete={handleDelete}
            />
            <StatusList
              status="Doing"
              description="This task is currently in progress."
              todos={doingTodos}
              onUpadate={handleUpdate}
              onDelete={handleDelete}
            />
            <StatusList
              status="Completed"
              description="This task is completed and done."
              todos={completedTodos}
              onUpadate={handleUpdate}
              onDelete={handleDelete}
            />
          </div>

          <DragOverlay>
            {draggingItem ? (
              <div
                style={{
                  padding: "8px",
                  backgroundColor: "white",
                  boxShadow: "0 1px 3px rgba(0,0,0,0.2)",
                  borderRadius: "4px",
                }}
              >
                <TodoItem
                  todo={draggingItem}
                  onUpdate={(vals) => handleUpdate(draggingItem.getId(), vals)}
                  onDelete={handleDelete.bind(undefined, draggingItem.getId())}
                />
              </div>
            ) : null}
          </DragOverlay>
        </DndContext>
      )}
    </section>
  );
}
