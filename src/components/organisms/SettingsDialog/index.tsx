import { HTMLAttributes } from "react";
import styles from "./style.module.scss";
import Button from "@/components/atoms/Button";
import Icon from "@/components/atoms/Icon";
import { FaCheck, FaPlus } from "react-icons/fa";
import { FaCircleXmark } from "react-icons/fa6";
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

  function onCancel() {
    settingsDialogRef.current?.close();
  }

  return (
    <dialog
      className={`${styles["settings-dialog"]} ${className ?? ""}`}
      ref={settingsDialogRef}
      {...props}
    >
      <section>
        <h2>Music Folders:</h2>
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
      <div className={styles["settings-dialog__buttons"]}>
        <Button onPress={onApply}>
          <Icon icon={FaCheck} />
          Apply
        </Button>

        <Button onPress={onCancel}>
          <Icon icon={FaCircleXmark} />
          Cancel
        </Button>
      </div>
    </dialog>
  );
};

export default SettingsDialog;
