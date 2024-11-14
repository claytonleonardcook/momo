import { invoke } from "@tauri-apps/api/core";
import styles from "./style.module.scss";
import TrackControls from "@/components/organisms/TrackControls";
import MediaGrid from "@/components/organisms/MediaGrid";

function Dashboard() {
  invoke("get_all_tracks").then(console.log).catch(console.error);

  return (
    <main className={styles["dashboard"]}>
      <TrackControls trackDuration={50} />
      <MediaGrid />
    </main>
  );
}

export default Dashboard;
