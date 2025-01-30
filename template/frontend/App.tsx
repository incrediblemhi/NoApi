import { add } from "@functions";
import { useState } from "react";

function App() {
  let [result, setResult] = useState(0);
  let [num, setNum] = useState(0);

  return (
    <>
      <main>
        <h1
          onClick={() => {
            add(num, result).then((res) => {
              setResult(res);
            });
          }}
          className="font-semibold text-2xl"
        >
          {result}
        </h1>
      </main>
    </>
  );
}

export default App;
