import styles from "./style.module.scss";
import TrackControls from "@/components/organisms/TrackControls";

function Dashboard() {
  return (
    <main className={styles["dashboard"]}>
      <TrackControls trackDuration={50} />
    </main>
  );
}

export default Dashboard;
