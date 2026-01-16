// presentation/pages/todo/TodoList.tsx
import { useTodo } from "@/application/todo/useTodo";
import { TodoForm } from "@/presentation/components/todo/TodoForm";
import { TodoItem } from "@/presentation/components/todo/TodoItem";
import { Alert } from "@/presentation/components/ui/Alert";
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { Todo } from "@/domains/todo/entities/Todo";

interface SortableTodoItemProps {
  todo: Todo;
  onUpdate: (
    id: string,
    updates: { status?: string; position?: number }
  ) => void;
  onDelete: (id: string) => void;
}

// 可排序的 Todo 项组件
function SortableTodoItem({ todo, onUpdate, onDelete }: SortableTodoItemProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({
    id: todo.getId(),
  });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
    backgroundColor: isDragging ? "#f0f0f0" : "white",
  };

  return (
    <div ref={setNodeRef} style={style} {...attributes} {...listeners}>
      <TodoItem todo={todo} onUpdate={onUpdate} onDelete={onDelete} />
    </div>
  );
}

// 单个状态分组列表
function StatusList({
  status,
  todos,
  onUpdate,
  onDelete,
}: {
  status: string;
  todos: Todo[];
  onUpdate: (
    id: string,
    updates: { status?: string; position?: number }
  ) => void;
  onDelete: (id: string) => void;
}) {
  return (
    <div className="mb-6">
      <h2 className="text-xl font-semibold mb-2 capitalize">{status}</h2>
      <SortableContext
        id={status}
        items={todos.map((todo) => todo.getId())}
        strategy={verticalListSortingStrategy}
      >
        {todos
          .sort((a, b) => a.getPosition() - b.getPosition())
          .map((todo) => (
            <SortableTodoItem
              key={todo.getId()}
              todo={todo}
              onUpdate={onUpdate}
              onDelete={onDelete}
            />
          ))}
      </SortableContext>
    </div>
  );
}

export function TodoList() {
  const { todos, isLoading, error, createTodo, updateTodo, deleteTodo } =
    useTodo();

  // 按状态分组
  const pendingTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "pending"
  );
  const doingTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "doing"
  );
  const completedTodos = todos.filter(
    (todo) => todo.getStatus().getValue() === "completed"
  );

  // 配置拖拽传感器
  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  // 处理拖拽结束
  const handleDragEnd = (event: any) => {
    const { active, over } = event;

    if (!over) return;

    const activeId = active.id;
    const overId = over.id;
    const activeStatus = active.data.current?.sortable.containerId;
    const overStatus = over.data.current?.sortable.containerId;

    if (activeId !== overId || activeStatus !== overStatus) {
      // 找到拖动的 Todo 和目标位置
      const allTodos = [...todos];
      const activeTodo = allTodos.find((todo) => todo.getId() === activeId);
      if (!activeTodo) return;

      // 计算新位置
      const targetTodos = allTodos.filter(
        (todo) => todo.getStatus().getValue() === overStatus
      );
      const overIndex = targetTodos.findIndex(
        (todo) => todo.getId() === overId
      );
      const newIndex = overIndex >= 0 ? overIndex : targetTodos.length;

      // 更新 status（如果跨状态）
      const updates: { status?: string; position?: number } = {};
      if (activeStatus !== overStatus) {
        updates.status = overStatus;
      }
      updates.position = newIndex;

      // 更新所有受影响的 Todo 的 position
      const reorderedTodos = allTodos.filter(
        (todo) => todo.getId() !== activeId
      );
      reorderedTodos.splice(
        reorderedTodos.filter((t) => t.getStatus().getValue() === overStatus)
          .length > 0
          ? reorderedTodos.findIndex(
              (t) => t.getStatus().getValue() === overStatus
            ) + newIndex
          : reorderedTodos.length,
        0,
        activeTodo
      );

      // 更新 position
      const updatesBatch = reorderedTodos
        .filter((todo) => todo.getStatus().getValue() === overStatus)
        .map((todo, index) => ({
          id: todo.getId(),
          position: index,
        }));

      // 如果是跨状态拖拽，先更新 status
      if (activeStatus !== overStatus) {
        updateTodo(activeId, updates);
      }

      // 批量更新 position
      updatesBatch.forEach(({ id, position }) => {
        updateTodo(id, { position });
      });
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">Todo 列表</h1>
      <TodoForm onSubmit={createTodo} />
      {error && <Alert variant="destructive">{error}</Alert>}
      {isLoading ? (
        <p>加载中...</p>
      ) : (
        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          onDragEnd={handleDragEnd}
        >
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <StatusList
              status="pending"
              todos={pendingTodos}
              onUpdate={updateTodo}
              onDelete={deleteTodo}
            />
            <StatusList
              status="doing"
              todos={doingTodos}
              onUpdate={updateTodo}
              onDelete={deleteTodo}
            />
            <StatusList
              status="completed"
              todos={completedTodos}
              onUpdate={updateTodo}
              onDelete={deleteTodo}
            />
          </div>
        </DndContext>
      )}
    </div>
  );
}
