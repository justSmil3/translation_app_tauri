import React, { useEffect, useState } from "react";
import { AiOutlineCheckCircle, AiOutlineCloudUpload } from "react-icons/ai";
import { MdClear } from "react-icons/md";
import "./App.css";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

const Dropfield = ({
  onFilesSelected,
  width,
  height,
}) => {
  const [files, setFiles] = useState([[],[]]);
  const [basePath, setBasePath] = useState("");
useEffect(() => {
		const unlisten = listen('tauri://file-drop', (event) => {
			invoke("get_all_files",{paths: event.payload});
		});

		return () => {
			unlisten.then(f => f());
		};
	}, []);

	useEffect(() => {
		const unlisten = listen("file_handled", (event) => {
			console.log(event.payload);
			if(files.length > 0) {
				let _filePaths = [];
				let _fileNames = [];
				files[0].map((file, index) => {
					if(file !== event.payload){
						_filePaths.push(file);
						_fileNames.push(files[1][index]);
					}
				})
				setFiles([_filePaths, _fileNames]);
			}
		})

		return () => {
			unlisten.then(f => f());
		};
	}, []);

	useEffect(() => {
		const filesEventListener = listen("files_listed", (event) =>{
			let new_files = event.payload.filter(x => !files.includes(x));
			console.log(event.payload);
			console.log(new_files.length);
			new_files.map((file, index) => console.log(file))
			
			setFiles((prevFiles) => [[...prevFiles[0], ...new_files[0]],[...prevFiles[1], ...new_files[1]]]);
		});

		return () => {
			filesEventListener.then(f => f());
		};

	}, []);
//const handleDrop = (event) => {
//    event.preventDefault();
//    const droppedFiles = event.dataTransfer.files;
//	//console.log(event);
//    if (droppedFiles.length > 0) {
//      const newFiles = Array.from(droppedFiles);
//		
//	
//      setFiles((prevFiles) => [...prevFiles, ...newFiles]);
//    }
//  };

	const handleRemoveFile = (index) => {
		
		setFiles((prevFiles) => [prevFiles[0].filter((_,i) => i !==index), prevFiles[1].filter((_,i) => i !== index)]);
	}

// 	const handleFileChange = (event) => {
// 		sendPath();
//     const selectedFiles = event.target.files;
//     if (selectedFiles && selectedFiles.length > 0) {
//       const newFiles = Array.from(selectedFiles);
//       setFiles((prevFiles) => [...prevFiles, ...newFiles]);
//     }
//   };
	

	const openFileDialog = async () => {
		const selectedPaths = await open({
			multiple: true,
			directory: false,
			filters: [
				{ name: 'Files', extensions: ['phtml', 'csv', 'php'] },
			],
		});

		if (selectedPaths) {
			if (Array.isArray(selectedPaths)) {
				await invoke('get_all_files', { paths: selectedPaths });
        
			} else {
				await invoke('get_all_files', { paths: [selectedPaths] });
			}
		}
	};

	const openFolderDialog = async () => {
		const selectedPaths = await open({
			multiple: false,
			directory: true,
		});

		if (selectedPaths) {
			if (Array.isArray(selectedPaths)) {
				await invoke('get_all_files', { paths: selectedPaths });
        
			} else {
				await invoke('get_all_files', { paths: [selectedPaths] });
			}
		}
	};



	useEffect(() => {
		onFilesSelected(files);
		console.log(files);
	}, [files, onFilesSelected]);

	return (
    <section className="drag-drop" style={{ width: width, height: height }}>
      <div
        className={`document-uploader ${
          files.length > 0 ? "upload-box active" : "upload-box"
        }`}
        onDragOver={(event) => event.preventDefault()}
      >
        <>
          <div className="upload-info">
            <AiOutlineCloudUpload />
            <div>
              <p>Drag and drop your files here</p>
              <p>
                Supported files: .CSV, .PHTML
              </p>
            </div>
          </div>
        </>

        {files.length > 0 && (
          <div className="file-list">
            <div className="">
			{files[1].map((file, index) => (
                <div className="file-item" key={index}>
                  <div className="file-info file-item__container">
				  {/*<p>{file.split(/[/\\]/).pop()}</p>*/}
				  <p>{file}</p>
                  </div>
                  <div className="file-actions">
                    <MdClear onClick={() => handleRemoveFile(index)} />
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {files.length > 0 && (
          <div className="success-file">
            <AiOutlineCheckCircle
              style={{ color: "#6DC24B", marginRight: 1 }}
            />
            <p>{files[0].length} file(s) selected</p>
          </div>
        )}
      </div>
		<div>
		<button className="browse-btn" onClick={openFileDialog}>add file</button>
			<button className="browse-btn" onClick={openFolderDialog}>add folder</button>
			<button className="browse-btn" onClick={()=>setFiles([[],[]])}>clear all</button>
		</div>
    </section>
  );	

};

export default Dropfield;
