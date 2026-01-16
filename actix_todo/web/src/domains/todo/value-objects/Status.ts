export type TodoStatus = "pending" | "doing" | "completed";

export default class Status {
  private constructor(private readonly value: TodoStatus) {}

  static create(value: TodoStatus): Status {
    if (!["pending", "doing", "completed"].includes(value)) {
      throw new Error("Invalid status");
    }
    return new Status(value);
  }

  getValue(): TodoStatus {
    return this.value;
  }

  equals(other: Status): boolean {
    return this.value === other.value;
  }
}
