import Description from "../value-objects/Description";
import Status from "../value-objects/Status";
import Timestamp from "@/domains/shared/value-objects/Timestamp";

export default class Todo {
  constructor(
    private readonly id: number,
    private description: Description,
    private status: Status,
    private position: number,
    private readonly createdAt: Timestamp,
    private updatedAt: Timestamp
  ) {}

  getId(): number {
    return this.id;
  }

  getDescription(): Description {
    return this.description;
  }

  getStatus(): Status {
    return this.status;
  }

  getCreatedAt(): Timestamp {
    return this.createdAt;
  }

  getUpdatedAt(): Timestamp {
    return this.updatedAt;
  }

  updatePosition(position: number): void {
    if (position < 0) {
      throw new Error("Position cannot be negative");
    }
    this.position = position;
    this.updatedAt = Timestamp.now();
  }
}