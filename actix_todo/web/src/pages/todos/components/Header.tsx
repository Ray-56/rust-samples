import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { PictureInPicture2, Plus } from "lucide-react";
import { useRef } from "react";

interface HeaderProps {
  onFinish?: (description?: string) => void;
}

export const Header: React.FC<HeaderProps> = (props) => {
  const { onFinish } = props;
  const refInput = useRef<HTMLInputElement>(null);

  return (
    <div id="pip-content" className="flex items-center gap-x-2">
      <Input
        type="text"
        ref={refInput}
        placeholder="Please enter a description of the task"
      />
      <Button
        onClick={() => {
          onFinish?.(refInput.current?.value);
          refInput.current!.value = "";
        }}
        className="cursor-pointer"
        size="icon"
      >
        <Plus />
      </Button>
      <Button
        size="icon"
        onClick={async () => {
          if (!document) return;

          const pipContent = document.querySelector("#pip-content");
          const pipWindow = await window.documentPictureInPicture.requestWindow(
            {
              width: 200, // 设置窗口的宽度
              height: 300, // 设置窗口的高度
            }
          );

          const description = document.createElement("div");
          description.textContent =
            "Picture-in-Picture Window. wait to implement PostMessage";
          pipWindow.document.body.appendChild(description);
          pipWindow.document.body.appendChild(pipContent!.cloneNode(true));

          // 设置 PiP 样式同步
          [...document.styleSheets].forEach((styleSheet) => {
            try {
              const cssRules = [...styleSheet.cssRules]
                .map((rule) => rule.cssText)
                .join("");
              const style = document.createElement("style");
              style.textContent = cssRules;
              pipWindow.document.head.appendChild(style);
            } catch (e) {
              const link = document.createElement("link");
              link.rel = "stylesheet";
              link.type = styleSheet.type;
              // link.media = styleSheet.media;
              link.href = styleSheet.href ?? "";
              pipWindow.document.head.appendChild(link);
            }
          });

          // TODO: 加入 PostMessage 通信
        }}
      >
        <PictureInPicture2 />
      </Button>
    </div>
  );
};
