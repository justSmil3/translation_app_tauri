import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import {sep} from "@tauri-apps/api/path";
import Dropfiled from "./dropfield.jsx";
import Dropdown from "./dropdown";
import MultiDropdown from "./MultiDropdown"
import ApiTextInput from "./ApiTextInput";
import ChooseOutput from "./ChooseOutput";
import "./App.css";



function App() {
	
	const serviceOptions = ["DeepL"];
  const options = 
		   ["English", 
			"Japanese", 
			"Chinese", 
			"Korean",
		    "Arabic",
			"Bulgarian",
    		"Czech",
    		"Danish",
    		"German",
    		"Greek",
    		"Spanish",
    		"Estonian",
    		"Finnish",
    		"French",
    		"Hungarian",
    		"Indonesian",
    		"Italian",
    		"Lithuanian",
    		"Latvian",
    		"Norwegian Bokmål",
    		"Dutch",
    		"Polish",
    		"Portuguese",
    		"Portuguese (Brazilian)",
    		"Romanian",
    		"Russian",
    		"Slovak",
    		"Slovenian",
    		"Swedish",
    		"Turkish",
    		"Ukrainian"];
  const [targetOptions, setTargetOptions] = useState([
			"Japanese", 
			"Chinese", 
			"Korean",
		    "Arabic",
			"Bulgarian",
    		"Czech",
    		"Danish",
    		"German",
    		"Greek",
    		"Spanish",
    		"Estonian",
    		"Finnish",
    		"French",
    		"Hungarian",
    		"Indonesian",
    		"Italian",
    		"Lithuanian",
    		"Latvian",
    		"Norwegian Bokmål",
    		"Dutch",
    		"Polish",
    		"Portuguese",
    		"Portuguese (Brazilian)",
    		"Romanian",
    		"Russian",
    		"Slovak",
    		"Slovenian",
    		"Swedish",
    		"Turkish",
    		"Ukrainian"]);
  const [selectetTargetOptions, setSelectedTargetOptions] = useState([]);
  const [sourceLanguage, setSourceLanguage] = useState("English");
  const [outputPath, setOuputPath] = useState("./translated/");

  const [files, setFiles] = useState([[],[]]);
	const [apiKey, setApiKey] = useState("");

  async function translate() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // setGreetMsg(await invoke("greet", { name }));
	  let outPath = outputPath;
	  if (!outPath.endsWith(sep)){
		  outPath += sep;
	  }
	  console.log("==========" +  outPath);
	  await invoke("translate_task", {files: files[0], fileNames: files[1], outputPath: outPath, languages: selectetTargetOptions, sourceLang: sourceLanguage });
  }

	const handleApiKeyChange = (key) => {
		
	};

	const handleOutChange = (text) => {
		setOuputPath(text);
	}

	const handleSourceLangSelect = (option) => {
		setSourceLanguage(option);
		let newTargets = options.filter(o => o !== option);
		setTargetOptions(newTargets);
	}
	const handleTargetLangSelect = (options) => {
		setSelectedTargetOptions(options);
		console.log(selectetTargetOptions);
	}

  return (
    <div className="container">
	  <div className="row">
		<div>
		  <div>
			<Dropfiled onFilesSelected={setFiles} width="300px" height="400px"/>	
		  </div>
	  </div>
		<div>
			<div>
				<lable>Source Language: </lable>
				<Dropdown options={options} onSelect={handleSourceLangSelect}/>
			</div>
			<div>
				<lable>Target Language: </lable>
				<MultiDropdown options={targetOptions} onSelect={handleTargetLangSelect}/>
			</div>
			<div>
				<lable>Output Path: </lable>
				<ChooseOutput defaultValue={outputPath} onChange={handleOutChange}/>
			</div>
			<div>
				<lable>Service: </lable>
				<Dropdown options={serviceOptions} onSelect={()=>{}} />
			</div>
			<div>
				<lable>API key: </lable>
				<ApiTextInput onChange={handleApiKeyChange} />	
			</div>
			<button onClick={translate}>translate</button>
	    </div>
	  </div>
    </div>
  );
}

export default App;
