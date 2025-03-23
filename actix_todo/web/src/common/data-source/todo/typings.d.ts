declare namespace API {
  interface TodoDatum {
    id: number;
    description: string;
    status: "pending" | "doing" | "completed";
    position: number;
    updated_at: number;
    created_at: number;
  }

  interface AddTodoDatumPayload {
    description: string;
  }
}
