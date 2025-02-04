import { add } from "@functions";
//import { useState } from "react";

function App() {
  return (
    <>
      <main>
        <h1
          onClick={() => {
            add("kelvin", "kelvin", "osei2323").then((_res) => {});
          }}
          className="font-semibold text-2xl"
        >
          "kev00000000000000"
        </h1>
      </main>
    </>
  );
}

export default App;
