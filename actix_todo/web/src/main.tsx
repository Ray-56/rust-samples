import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import { SWRConfig } from "swr";

import App from "./App.tsx";
import "./global.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <SWRConfig
      value={{
        onError: (error) => {
          // TODO: Test whether API errors are caught
          // Here you can add error handling logic, such as recording error logs, error prompts, etc.
          console.error("SWR Error:", error);
        },
      }}
    >
      <App />
    </SWRConfig>
  </BrowserRouter>
);
