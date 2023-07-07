import {PanelSection, PanelSectionRow, quickAccessControlsClasses, Field, TextField, ButtonItem, ServerAPI} from "decky-frontend-lib"
import { VFC, Fragment, useState, useEffect } from "react"

interface AppData {
  steamid?: string;
  gameTitle: string;
}

interface GptQuestion {
   gameTitle: string;
   question: string;
}

interface GptAnswer {
  question: string;
  answer: string;
  appData: AppData;
}

export const AskQuestions: VFC<{ serverApi: ServerAPI }> = ({ serverApi }) => {
    // logic for calling sidefx goes here
   const [question, setQuestion] = useState<GptQuestion | undefined>();
   const [appData, setAppData] = useState<AppData | undefined>();
   const [answers, setAnswers] = useState<GptAnswer[]>([]);

   const getSteamAppData = async () => {
    const result = await serverApi.callPluginMethod<{}, AppData>(
      "get_running_app_data",
      {}
    );
    if (result.success) {
      setAppData(result.result);
    }
   }

   useEffect(() => {
    getSteamAppData();
  }, []);

   const askGPT = async () => {
     const result = await serverApi.callPluginMethod<GptQuestion, GptAnswer>(
       "ask_gpt",
       question ?? { gameTitle: "", question: "" }
     );
     if (result.success) {
        setAnswers([result?.result, ...answers]);
     } else {
        //setAnswer({text: "oh no!!!! result: " + result.result});
     }
   };

    return (
      <>
        <style>
        {`
          .questions-scoper .${quickAccessControlsClasses.PanelSection} {
            width: inherit;
            height: inherit;
            padding: 0px;
          }
        `}
        </style>
        <div className="questions-scoper">
            <PanelSection>
                <PanelSectionRow>
                    <Field
                      label={ appData?.gameTitle ? "Ask ChatGPT anything about " + appData?.gameTitle :"Ask ChatGPT anything about anything."}
                      description={
                        <TextField
                          onChange={(e) => setQuestion( { gameTitle: appData?.gameTitle ?? "", question: e?.target.value } )}
                        />
                      }
                    />
                    <ButtonItem
                      onClick={() => {askGPT()}}>
                        Submit
                    </ButtonItem>
                </PanelSectionRow>
                {answers.map((answer) => (
                  <PanelSectionRow>
                    <Field label={answer.question}>
                      Answer: {answer.answer}
                    </Field>
                  </PanelSectionRow>
                ))}
            </PanelSection>
        </div>
     </>
    );
}
