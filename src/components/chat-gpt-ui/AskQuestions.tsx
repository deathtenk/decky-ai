import {PanelSection, PanelSectionRow, quickAccessControlsClasses, Field, TextField, ButtonItem, ServerAPI} from "decky-frontend-lib"
import { VFC, Fragment, useState } from "react"

interface GptQuestion {
   gameTitle: string;
   question: string;
}

interface GptAnswer {
  text: string;
}

export const AskQuestions: VFC<{ serverApi: ServerAPI }> = ({ serverApi }) => {
    // logic for calling sidefx goes here
   const [answer, setAnswer] = useState<GptAnswer | undefined>();
   const [question, setQuestion] = useState<GptQuestion | undefined>();
   //setQuestion({gameTitle: "", question: ""});

   const askGPT = async () => {
     const result = await serverApi.callPluginMethod<GptQuestion, GptAnswer>(
       "ask_gpt",
       question ?? { gameTitle: "", question: "" }
     );
     if (result.success) {
       setAnswer(result.result);
     } else { 
        setAnswer({text: "oh no!!!! result: " + result.result});
     }
   };

   //askGPT( {gameTitle: "Skyrim",
   //         question: "How do you purchase a horse?"} );

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
                      label="Ask ChatGPT Anything About <insert-game-name-here>"
                      description={
                        <TextField
                          onChange={(e) => setQuestion( { gameTitle: "Skyrim", question: e?.target.value } )}
                        />
                      }
                    />
                    <ButtonItem
                      onClick={() => {askGPT()}}>
                        Submit
                    </ButtonItem>
                </PanelSectionRow>
                <PanelSectionRow>
                  <Field label={answer?.text ?? "nothing here but us chickens"}/>
                </PanelSectionRow>
            </PanelSection>
        </div>
     </>
    );
} 