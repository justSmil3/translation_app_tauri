import React, { useState } from 'react';
import './TextInput.css';
import { open } from '@tauri-apps/api/dialog';
import {sep} from '@tauri-apps/api/path';


const ChooseOutput = ({onChange, defaultValue}) => {
  const [value, setValue] = useState(defaultValue);

  const handleChange = (event) => {
    setValue(event.target.value);
    onChange(event.target.value);
  };


	const openFolderDialog = async () => {
		const selectedPath = await open({
			multiple: false,
			directory: true,
		});
		let new_path = selectedPath + sep;
		setValue(new_path);
		onChange(new_path);
	};

  return (
    <div className="text-input">
	  <input
        type="text"
        value={value}
        onChange={handleChange}
        placeholder="Enter Output Path ..."
      />
	  <button onClick={openFolderDialog}>browse</button>
    </div>
  );
};

export default ChooseOutput;

