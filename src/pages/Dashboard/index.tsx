import { useState } from "react";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import Button from "@/components/atoms/Button";
import styles from "./style.module.scss";
import { useToast } from "@/hooks/useToast";
import TrackControls from "@/components/molecules/TrackControls";

function Dashboard() {
  const { addToast } = useToast();
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // function greet() {
  //   addToast("Greeting you :)", "blue");

  //   invoke("greet", { name })
  //     .then((message) => setGreetMsg(message as string))
  //     .catch(console.error);
  // }

  return (
    <main className={styles["dashboard"]}>
      <TrackControls trackDuration={50} />
    </main>
  );
}

export default Dashboard;
