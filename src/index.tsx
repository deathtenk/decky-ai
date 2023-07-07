import {
  ButtonItem,
  TextField,
  definePlugin,
  DialogButton,
  Menu,
  MenuItem,
  PanelSection,
  PanelSectionRow,
  Router,
  ServerAPI,
  showContextMenu,
  staticClasses,
  Navigation,
  SidebarNavigation,
} from "decky-frontend-lib";
import { VFC, Fragment } from "react";
import { FaShip } from "react-icons/fa";
import { AskQuestions } from "./components/chat-gpt-ui/AskQuestions";

import logo from "../assets/logo.png";

// interface AddMethodArgs {
//   left: number;
//   right: number;
// }

const Content: VFC<{ serverAPI: ServerAPI }> = ({}) => {
  // const [result, setResult] = useState<number | undefined>();

  // const onClick = async () => {
  //   const result = await serverAPI.callPluginMethod<AddMethodArgs, number>(
  //     "add",
  //     {
  //       left: 2,
  //       right: 2,
  //     }
  //   );
  //   if (result.success) {
  //     setResult(result.result);
  //   }
  // };

  return (
    <PanelSection>
      <PanelSectionRow/>
      <PanelSectionRow>
        <ButtonItem 
          layout="below" 
          onClick={() => { 
            Navigation.CloseSideMenus(); 
            Navigation.Navigate("/gpt-menu");
          }}
        >
              ChatGPT Questions
        </ButtonItem>
      </PanelSectionRow>
    </PanelSection>
  );
};

const ChatGPTMenu: VFC<{ serverApi: ServerAPI }> = ({ serverApi }) => {
  return (
    <SidebarNavigation
      title="Chat GPT Questions"
      showTitle
      pages={[
        {
          title: "Ask Questions",
          content: <AskQuestions serverApi={serverApi} />,
          route: "/gpt-menu/ask"
        }
      ]}/>
  );
};

export default definePlugin((serverApi: ServerAPI) => {
  serverApi.routerHook.addRoute("/gpt-menu", () => (<ChatGPTMenu serverApi={serverApi}/>));

  return {
    title: <div className={staticClasses.Title}>Decky AI</div>,
    content: <Content serverAPI={serverApi} />,
    icon: <FaShip />,
    onDismount() {
      serverApi.routerHook.removeRoute("/gpt-menu");
    },
  };
});
