import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Button from "@/components/atoms/Button";
import styles from "./style.module.scss";

function Dashboard() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  function greet() {
    invoke("greet", { name })
      .then((message) => setGreetMsg(message as string))
      .catch(console.error);
  }

  return (
    <main className={styles["dashboard"]}>
      <h1>Welcome to Tauri!</h1>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <Button type={"submit"}>Greet</Button>
      </form>

      <p>{greetMsg}</p>
    </main>
  );
}

export default Dashboard;
