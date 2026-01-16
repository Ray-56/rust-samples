import { Username } from "../value-objects/Username";

export class User {
  constructor(
    private readonly id: string,
    private username: Username
  ) {}

  getId(): string {
    return this.id;
  }

  getUsername(): Username {
    return this.username;
  }
}
