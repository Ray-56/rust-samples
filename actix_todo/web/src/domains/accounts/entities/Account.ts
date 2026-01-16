export class Account {
  constructor(private readonly id: number, private readonly username: string) {}

  getId(): number {
    return this.id;
  }

  getUsername(): string {
    return this.username;
  }
}
