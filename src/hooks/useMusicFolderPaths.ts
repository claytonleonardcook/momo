import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

function useMusicFolderPaths() {
  const [paths, setPaths] = useState<string[]>([]);

  useEffect(() => {
    invoke("get_music_folder_paths")
      .then((value) => setPaths(value as string[]))
      .catch(console.error);
  }, []);

  function addPath(path: string) {
    invoke("add_music_folder_path", {
      path,
    })
      .then((value) => setPaths(value as string[]))
      .catch(console.error);
  }

  return {
    paths,
    addPath,
  };
}

export default useMusicFolderPaths;
