import styles from "./style.module.scss";
import TrackControls from "@/components/organisms/TrackControls";
import MediaGrid from "@/components/organisms/MediaGrid";

function Dashboard() {
  return (
    <main className={styles["dashboard"]}>
      <TrackControls trackDuration={50} />
      <MediaGrid className={styles["dashboard__media-grid"]} />
    </main>
  );
}

export default Dashboard;
