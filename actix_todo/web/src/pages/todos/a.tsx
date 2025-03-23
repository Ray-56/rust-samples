import {
  useCallback,
  useContext,
  useEffect,
  useReducer,
  useState,
} from "react";
import { GripVertical } from "lucide-react";
import {
  CollisionDetection,
  DndContext,
  DragEndEvent,
  DragOverlay,
  MeasuringStrategy,
  UniqueIdentifier,
} from "@dnd-kit/core";
import {
  SortableContext,
  sortableKeyboardCoordinates,
  arrayMove,
} from "@dnd-kit/sortable";

import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import Todo from "@/common/domains/todo-domain/entities/todo";
import {
  Header,
  TodoItem,
  FormValues,
  Draggable,
  Droppable,
  SortableItem,
} from "./components";
import { ActionType, initializerState, reducer, TodosCtx } from "./context";

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

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;
    if (over && active.id !== over.id) {
      const activeIndex = ctx.state.todos.pending.findIndex(todo => todo.getId() === active.id);
      const overIndex = ctx.state.todos.pending.findIndex(todo => todo.getId() === over.id);

      ctx.dispatch({
        type: ActionType.ReorderTodos,
        payload: {
          from: activeIndex,
          to: overIndex,
        },
      });
    }
    setActiveId(null);
    setDraggingItem(null);
  }

  return (
    <section className="p-4">
      <Header onFinish={handleAdd} />

      <DndContext
        measuring={{
          droppable: { strategy: MeasuringStrategy.Always },
        }}
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
        <SortableContext items={ctx.state.todos.pending}>
          <div className="mt-4 grid gap-3 grid-cols-[repeat(auto-fill,minmax(350px,1fr))]">
            <Card className="flex-1">
              <CardHeader>
                <CardTitle>Pending</CardTitle>
                <CardDescription>
                  This task is pending and not started yet.
                </CardDescription>
              </CardHeader>
              <CardContent className="bg-gray-100">
                <Droppable id="pending">
                  {ctx.state.todos.pending.map((todo) => (
                    <SortableItem
                      key={todo.getId()}
                      id={todo.getId()}
                      className="cursor-move"
                      style={{
                        opacity: activeId === todo.getId() ? 0.5 : 1,
                      }}
                    >
                      <TodoItem
                        data={todo}
                        key={todo.getId()}
                        onUpdate={(vals) => handleUpdate(todo, vals)}
                        onDelete={handleDelete.bind(undefined, todo)}
                        prefix={
                          <Button
                            size="icon"
                            variant="ghost"
                            className="cursor-move"
                          >
                            <GripVertical />
                          </Button>
                        }
                      />
                    </SortableItem>
                  ))}
                </Droppable>
              </CardContent>
            </Card>
            <Card className="flex-1">
              <CardHeader>
                <CardTitle>Doing</CardTitle>
                <CardDescription>
                  This task is currently in progress.
                </CardDescription>
              </CardHeader>
              <CardContent className="">
                <Droppable id="doing">
                  {ctx.state.todos.doing.map((todo) => (
                    <TodoItem
                      data={todo}
                      key={todo.getId()}
                      onUpdate={(vals) => handleUpdate(todo, vals)}
                      onDelete={handleDelete.bind(undefined, todo)}
                      prefix={
                        <Button
                          size="icon"
                          variant="ghost"
                          className="cursor-move"
                        >
                          <GripVertical />
                        </Button>
                      }
                    />
                  ))}
                </Droppable>
              </CardContent>
            </Card>
            <Card className="flex-1">
              <CardHeader>
                <CardTitle>Completed</CardTitle>
                <CardDescription>
                  This task is completed and done.
                </CardDescription>
              </CardHeader>
              <CardContent className="flex flex-col gap-y-2">
                <Droppable id="completed">
                  {ctx.state.todos.completed.map((todo) => (
                    <TodoItem
                      data={todo}
                      key={todo.getId()}
                      onUpdate={(vals) => handleUpdate(todo, vals)}
                      onDelete={handleDelete.bind(undefined, todo)}
                      prefix={
                        <Button
                          size="icon"
                          variant="ghost"
                          className="cursor-move"
                        >
                          <GripVertical />
                        </Button>
                      }
                    />
                  ))}
                </Droppable>
              </CardContent>
            </Card>
          </div>
        </SortableContext>

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
                data={draggingItem}
                onUpdate={(vals) => handleUpdate(draggingItem, vals)}
                onDelete={handleDelete.bind(undefined, draggingItem)}
                prefix={
                  <Button size="icon" variant="ghost" className="cursor-move">
                    <GripVertical />
                  </Button>
                }
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