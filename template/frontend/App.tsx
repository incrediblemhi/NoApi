import { hello } from "@functions";
import { useState } from "react";

function App() {
  let [name, setName] = useState(0);

  return (
    <>
      <main>
        <h1
          onClick={() => {
            hello(1.555555, 1.5).then((res) => {
              setName(res);
            });
          }}
          className="font-semibold text-2xl"
        >
          {name}
        </h1>
      </main>
    </>
  );
}

export default App;
