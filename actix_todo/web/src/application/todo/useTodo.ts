import useSWR, { useSWRConfig } from "swr";
import TodoService from "@/domains/todo/services/TodoService";
import { TodoDTO } from "@/domains/todo/types";

export const useTodo = () => {
  const {
    data: todos = [],
    error,
    isLoading,
    mutate,
  } = useSWR("todos", TodoService.getTodos);

  const { mutate: globalMutate } = useSWRConfig();

  async function createTodo(description: string) {
    try {
      const newTodo = await TodoService.createTodo(description);
      mutate([...todos, newTodo], false); // Optimistic update
      // Re-acquire data and keep it consistent with the server
      globalMutate("todos");
    } catch (err) {
      throw new Error("Failed to create todo, err: " + err);
    }
  }

  async function updateTodo(
    id: number,
    data: Partial<Pick<TodoDTO, "description" | "position" | "status">>
  ) {
    try {
      const updatedTodo = await TodoService.updateTodo(id, data);
      const newTodos = todos.map((todo) =>
        todo.getId() === id ? updatedTodo : todo
      );
      mutate(newTodos, false); // Optimistic update
      globalMutate("todos");
    } catch (err) {
      throw new Error("Failed to update todo, err: " + err);
    }
  }

  async function deleteTodo(id: number) {
    try {
      await TodoService.deleteTodo(id);
      const newTodos = todos.filter((todo) => todo.getId() !== id);
      mutate(newTodos, false); // Optimistic update
      globalMutate("todos");
    } catch (err) {
      throw new Error("Failed to delete todo, err: " + err);
    }
  }

  async function reorderTodo(status: TodoDTO["status"]) {
    try {
      const newTodos = await TodoService.reorderTodo(status);
      mutate(newTodos, false); // Optimistic update
      globalMutate("todos");
    } catch (err) {
      throw new Error("Failed to reorder todo, err: " + err);
    }
  }

  return {
    todos,
    isLoading,
    createTodo,
    updateTodo,
    deleteTodo,
    reorderTodo,
    error: error ? error.message : null,
  };
};
