export default class Todo {
  private id: number;
  private description: string;
  private status: API.TodoDatum["status"];
  private position: number;
  private created_at: number;
  private updated_at: number;
  constructor(datum: API.TodoDatum) {
    this.id = datum.id;
    this.description = datum.description;
    this.status = datum.status;
    this.position = datum.position;
    this.created_at = datum.created_at;
    this.updated_at = datum.updated_at
  }

  getId() {
    return this.id;
  }

  getDescription() {
    return this.description;
  }

  getStatus() {
    return this.status;
  }

  getPosition() {
    return this.position;
  }

  getCreatedAt() {
    return this.created_at;
  }

  getUpdatedAt() {
    return this.updated_at;
  }

  clone() {
    return new Todo({
      id: this.id,
      description: this.description,
      status: this.status,
      position: this.position,
      updated_at: this.updated_at,
      created_at: this.created_at
    });
  }

  patch(data: Partial<API.TodoDatum>) {
    if (data.description) {
      this.description = data.description;
    }
    if (data.status) {
      this.status = data.status;
    }
    if (data.position) {
      this.position = data.position;
    }
    if (data.id) {
      this.id = data.id;
    }
  }
}
