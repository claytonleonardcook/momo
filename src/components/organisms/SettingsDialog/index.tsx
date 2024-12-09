import { HTMLAttributes } from "react";
import styles from "./style.module.scss";
import Button from "@/components/atoms/Button";
import Icon from "@/components/atoms/Icon";
import { FaCheck, FaPlus } from "react-icons/fa";
import useMusicFolderPaths from "@/hooks/useMusicFolderPaths";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import useSettingsDialog from "@/hooks/useSettingsDialog";

namespace SettingsDialog {
  export type Props = HTMLAttributes<HTMLDialogElement> & {
    open?: boolean;
  };
}

const SettingsDialog = ({ className, ...props }: SettingsDialog.Props) => {
  const settingsDialogRef = useSettingsDialog();
  const { paths, addPath } = useMusicFolderPaths();

  async function onAddFolder() {
    const directory = await openDialog({
      multiple: false,
      directory: true,
    });

    if (!directory) return;

    addPath(directory);
  }

  function onApply() {
    settingsDialogRef.current?.close();
  }

  return (
    <dialog
      className={`${styles["settings-dialog"]} ${className ?? ""}`}
      ref={settingsDialogRef}
      aria-labelledby={"dialog-title"}
      {...props}
    >
      <div>
        <header>
          <h2 id="dialog-title">Settings</h2>
        </header>
        <main>
          <section className={styles["settings-dialog__music-folders"]}>
            <h3>Music Folders:</h3>
            <ul>
              {paths.map((path, index) => (
                <li key={index}>{path}</li>
              ))}
            </ul>
            <Button onPress={onAddFolder}>
              <Icon icon={FaPlus} />
              Add New Folder
            </Button>
          </section>
          <Button
            onPress={onApply}
            className={styles["settings-dialog__apply"]}
          >
            <Icon icon={FaCheck} />
            Apply
          </Button>
        </main>
      </div>
    </dialog>
  );
};

export default SettingsDialog;
