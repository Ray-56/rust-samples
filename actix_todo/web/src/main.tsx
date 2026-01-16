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
          // TODO: 测试是否捕获到 API 的错误
          // 这里可以添加错误处理逻辑，比如记录错误日志、错误提示等
          console.error("SWR Error:", error);
        },
      }}
    >
      <App />
    </SWRConfig>
  </BrowserRouter>
);
