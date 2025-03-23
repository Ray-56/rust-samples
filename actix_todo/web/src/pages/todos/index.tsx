import { useContext, useEffect, useReducer, useState } from "react";
import {
  DndContext,
  DragEndEvent,
  DragOverlay,
  MeasuringStrategy,
  UniqueIdentifier,
} from "@dnd-kit/core";
import { arrayMove, SortableContext } from "@dnd-kit/sortable";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Todo from "@/common/domains/todo-domain/entities/todo";
import { Header, TodoItem, FormValues, Droppable } from "./components";
import { ActionType, initializerState, reducer, TodosCtx } from "./context";
import "./index.css";
import { reorderByIds } from "./utils";
import { Button } from "@/components/ui/button";
import { reorderTodo } from "@/common/data-source/todo/controller";

function Inner() {
  const ctx = useContext(TodosCtx);
  const [activeId, setActiveId] = useState<UniqueIdentifier | null>(null);
  const [draggingItem, setDraggingItem] = useState<Todo | null>(null);

  const handleAdd = async (description: string = "todo...") => {
    await ctx.state.todoServer.addTodo(description);
    ctx.dispatch({ type: ActionType.AddTodo, description });
  };

  const handleUpdate = async (todo: Todo, newVals: FormValues) => {
    const oldTodo = todo.clone();
    await ctx.state.todoServer.patchTodo(todo.getId(), newVals);

    ctx.dispatch({
      type: ActionType.PatchTodo,
      payload: { oldTodo, newTodo: todo },
    });
  };

  const handleDelete = async (todo: Todo) => {
    await ctx.state.todoServer.deleteTodo(todo.getId());
    ctx.dispatch({ type: ActionType.DeleteTodo, todo });
  };

  async function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (!active || !over) return;

    const { pending } = ctx.state.todos;
    const pendingIds = ctx.state.todos.pending.map((x) => x.getId());
    const activeIndex = pendingIds.indexOf(active.id as number);
    const overIndex = pendingIds.indexOf(over.id as number);

    const newPendingIds = arrayMove(pendingIds, activeIndex, overIndex);
    console.log(pendingIds, newPendingIds);
    const newPending = reorderByIds(ctx.state.todos.pending, newPendingIds);
    const newIdx = newPendingIds.indexOf(active.id as number);
    const target = newPending[newIdx];
    const prev = newPending[newIdx - 1]?.getPosition() ?? 0;
    const next = newPending[newIdx + 1]
      ? newPending[newIdx + 1].getPosition()
      : pending[pending.length - 1].getPosition() + 1000;
    const position = Math.floor((prev + next) / 2);
    console.log("newIdx", newIdx, "prev", prev, "next", next);
    console.log("position", position);
    target?.patch({ position });

    console.log("pending", ctx.state.todos.pending, "newPending", newPending);
    ctx.state.todoServer.repositionTodo(activeId as number, position);
    if (position === 0) {
      await ctx.state.todoServer.reorderTodos("pending");
    }
    ctx.dispatch({
      type: ActionType.RepositionTodo,
      payload: { type: "pending", todos: newPending },
    });
    // // TODO: 待验证
    // ctx.dispatch({ type: ActionType.RepositionTodo, todo: target! });
  }

  return (
    <section className="p-4">
      <Button
        onClick={async () => {
          reorderTodo("pending").then(console.log);
        }}
      >
        Reorder Pending Todos
      </Button>
      <Header onFinish={handleAdd} />

      <DndContext
        measuring={{ droppable: { strategy: MeasuringStrategy.Always } }}
        onDragStart={(event) => {
          setActiveId(event.active.id);
          console.log("start", event);
          const item = ctx.state.todoServer.findById(event.active.id as number);
          setDraggingItem(item || null);
        }}
        onDragOver={(event) => {
          console.log("over", event);
        }}
        onDragEnd={handleDragEnd}
      >
        <div className="mt-4 grid gap-3 grid-cols-[repeat(auto-fill,minmax(350px,1fr))]">
          <Card className="flex-1">
            <CardHeader>
              <CardTitle>Pending</CardTitle>
              <CardDescription>
                This task is pending and not started yet.
              </CardDescription>
            </CardHeader>
            <CardContent className="bg-gray-100">
              <SortableContext
                items={ctx.state.todos.pending.map((x) => x.getId())}
              >
                <Droppable id="pending">
                  {ctx.state.todos.pending.map((todo) => (
                    <TodoItem
                      key={todo.getId()}
                      id={todo.getId()}
                      data={todo}
                      onUpdate={(vals) => handleUpdate(todo, vals)}
                      onDelete={handleDelete.bind(undefined, todo)}
                    />
                  ))}
                </Droppable>
              </SortableContext>
            </CardContent>
          </Card>
          <Card className="flex-1">
            <CardHeader>
              <CardTitle>Doing</CardTitle>
              <CardDescription>
                This task is currently in progress.
              </CardDescription>
            </CardHeader>
            <CardContent className="bg-gray-100">
              <SortableContext
                items={ctx.state.todos.doing.map((x) => x.getId())}
              >
                <Droppable id="doing">
                  {ctx.state.todos.doing.map((todo) => (
                    <TodoItem
                      key={todo.getId()}
                      id={todo.getId()}
                      data={todo}
                      onUpdate={(vals) => handleUpdate(todo, vals)}
                      onDelete={handleDelete.bind(undefined, todo)}
                    />
                  ))}
                </Droppable>
              </SortableContext>
            </CardContent>
          </Card>
          <Card className="flex-1">
            <CardHeader>
              <CardTitle>Completed</CardTitle>
              <CardDescription>
                This task is completed and done.
              </CardDescription>
            </CardHeader>
            <CardContent className="bg-gray-100">
              <SortableContext
                items={ctx.state.todos.completed.map((x) => x.getId())}
              >
                <Droppable id="completed">
                  {ctx.state.todos.completed.map((todo) => (
                    <TodoItem
                      key={todo.getId()}
                      id={todo.getId()}
                      data={todo}
                      onUpdate={(vals) => handleUpdate(todo, vals)}
                      onDelete={handleDelete.bind(undefined, todo)}
                    />
                  ))}
                </Droppable>
              </SortableContext>
            </CardContent>
          </Card>
        </div>
        {/* </SortableContext> */}

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
                id={draggingItem.getId?.()}
                data={draggingItem}
                onUpdate={(vals) => handleUpdate(draggingItem, vals)}
                onDelete={handleDelete.bind(undefined, draggingItem)}
              />
            </div>
          ) : null}
        </DragOverlay>
      </DndContext>
    </section>
  );
}

export default function TodosPage() {
  const [state, dispatch] = useReducer(reducer, initializerState);

  useEffect(() => {
    state.todoServer.setup().then(() => {
      const pending = state.todoServer.getPendingTodos();
      const doing = state.todoServer.getDoingTodos();
      const completed = state.todoServer.getCompletedTodos();
      dispatch({
        type: ActionType.SetupTodos,
        payload: { pending, doing, completed },
      });
    });
  }, []);

  return (
    <TodosCtx.Provider value={{ state, dispatch }}>
      <Inner />
    </TodosCtx.Provider>
  );
}
