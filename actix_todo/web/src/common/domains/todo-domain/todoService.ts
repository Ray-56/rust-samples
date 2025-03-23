import Todo from "./entities/todo";
import * as controller from "@/common/data-source/todo/controller";

export default class TodoService {
  private todos: Todo[] = [];
  private setuped = false;

  constructor() {
    // controller.getTodos().then((rsp) => {
    //   this.todos = rsp.data.map((todo) => new Todo(todo));
    // });
  }

  async setup() {
    const rsp = await controller.getTodos();
    this.setuped = true;
    if (rsp.status === 200) {
      this.todos = rsp.data.map((todo) => new Todo(todo));
    }
  }

  findById(id: number) {
    return this.todos.find((todo) => todo.getId() === id);
  }

  async addTodo(description: string) {
    const rsp = await controller.addTodo({ description });
    if (rsp.status === 200) {
      this.todos.push(new Todo(rsp.data));
    }
  }

  /**
   * 更新 todo，执行后会更新对应的 todo 实例
   */
  async patchTodo(id: number, data: Partial<Omit<API.TodoDatum, "id">>) {
    const rsp = await controller.patchTodo(id, data);
    if (rsp.status === 200) {
      const target = this.todos.find((todo) => todo.getId() === id);
      if (target) {
        target.patch(data);
      }
    }
  }

  async deleteTodo(id: number) {
    const rsp = await controller.deleteTodo(id);
    if (rsp.status === 204) {
      this.todos = this.todos.filter((todo) => todo.getId() !== id);
    }
  }

  /** 重新定位 */
  async repositionTodo(id: number, position: number) {
    await this.patchTodo(id, { position: position });
  }

  /** 重整排序的序号 */
  async reorderTodos(status: API.TodoDatum["status"]) {
    await controller.reorderTodo(status);
  }

  getTodos() {
    if (!this.setuped) {
      throw new Error("Please setup first");
    }
    return this.todos;
  }

  getPendingTodos() {
    if (!this.setuped) {
      throw new Error("Please setup first");
    }
    return this.todos
      .filter((todo) => todo.getStatus() === "pending")
      .sort((a, b) => a.getPosition() - b.getPosition());
  }

  getDoingTodos() {
    if (!this.setuped) {
      throw new Error("Please setup first");
    }
    return this.todos.filter((todo) => todo.getStatus() === "doing");
  }

  getCompletedTodos() {
    if (!this.setuped) {
      throw new Error("Please setup first");
    }
    return this.todos.filter((todo) => todo.getStatus() === "completed");
  }
}
