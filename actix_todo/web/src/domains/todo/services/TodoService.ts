import { deleteTodo } from "@/common/data-source/todo/controller";
import Todo from "../entities/Todo";
import { TodoDTO } from "../types";
import Description from "../value-objects/Description";
import Status from "../value-objects/Status";
import Timestamp from "@/domains/shared/value-objects/Timestamp";
import * as apis from "@/infrastructure/api/todo/api";

export default class TodoService {
  static async createTodo(description: string): Promise<Todo> {
    const desc = Description.create(description);
    const status = Status.create("pending");
    const createdAt = Timestamp.now();
    const updatedAt = createdAt;
    const rsp = await apis.addTodo({ description: desc.getValue() });
    return new Todo(rsp.id, desc, status, rsp.position, createdAt, updatedAt);
  }

  static async updateTodo(
    id: number,
    data: Partial<Pick<TodoDTO, "description" | "position" | "status">>
  ) {
    const todo = await apis.patchTodo(id, data);
    return new Todo(
      todo.id,
      Description.create(todo.description),
      Status.create(todo.status),
      todo.position,
      Timestamp.create(todo.created_at),
      Timestamp.create(todo.updated_at)
    );
  }

  static async deleteTodo(id: number): Promise<void> {
    await deleteTodo(id);
  }

  static async getTodos(): Promise<Todo[]> {
    const rsp = await apis.getTodos();
    return rsp.map(
      (x) =>
        new Todo(
          x.id,
          Description.create(x.description),
          Status.create(x.status),
          x.position,
          Timestamp.create(x.created_at),
          Timestamp.create(x.updated_at)
        )
    );
  }

  static async reorderTodo(status: TodoDTO["status"]): Promise<Todo[]> {
    const rsp = await apis.reorderTodo(status);
    return rsp.map(
      (x) =>
        new Todo(
          x.id,
          Description.create(x.description),
          Status.create(x.status),
          x.position,
          Timestamp.create(x.created_at),
          Timestamp.create(x.updated_at)
        )
    );
  }

  static getTodoById(id: number, todos: Todo[]): Todo {
    const todo = todos.find((todo) => todo.getId() === id);
    if (!todo) {
      throw new Error(`Todo with id ${id} not found`);
    }
    return todo;
  }
}
