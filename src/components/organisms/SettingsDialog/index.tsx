import { HTMLAttributes, useContext } from "react";
import styles from "./style.module.scss";
import Button from "@/components/atoms/Button";
import Icon from "@/components/atoms/Icon";
import { FaCheck, FaPlus } from "react-icons/fa";
import useMusicFolderPaths from "@/hooks/useMusicFolderPaths";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import useSettingsDialog from "@/hooks/useSettingsDialog";
import RadioGroup from "@/components/atoms/RadioGroup";
import { ThemeContext, ToastContext } from "@/contexts";

namespace SettingsDialog {
  export type Props = HTMLAttributes<HTMLDialogElement> & {
    open?: boolean;
  };
}

const SettingsDialog = ({ className, ...props }: SettingsDialog.Props) => {
  const settingsDialogRef = useSettingsDialog();
  const { paths, addPath } = useMusicFolderPaths();

  const { theme, setLight, setDark } = useContext(ThemeContext);
  const { addToast } = useContext(ToastContext);

  async function onAddFolder() {
    const directory = await openDialog({
      multiple: false,
      directory: true,
    });

    if (!directory) return;

    addPath(directory);
  }

  function onThemeChange(value: string) {
    if (value === "light") setLight();
    else setDark();
  }

  function onApply() {
    settingsDialogRef.current?.close();
    addToast("Saved new settings.", "blue");
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
          <div>
            <section>
              <h3>Theme:</h3>
              <RadioGroup defaultValue={theme} onChange={onThemeChange}>
                <RadioGroup.RadioButton value={"light"}>
                  Light
                </RadioGroup.RadioButton>
                <RadioGroup.RadioButton value={"dark"}>
                  Dark
                </RadioGroup.RadioButton>
              </RadioGroup>
            </section>
            <section>
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
          </div>
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
