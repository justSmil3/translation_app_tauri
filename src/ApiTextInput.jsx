import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './TextInput.css';

const ApiTextInput = ({onChange }) => {
  const [value, setValue] = useState('');
  const [storedValue, setStoredValue] = useState('');

	useEffect(() => {
		const fetchApiKey = async () => {
			const key = await invoke('get_api_key');
			if(key){
				setStoredValue(key);
				setValue(key);
				onChange(key);
			}
		};

		fetchApiKey();
	}, [])

  const handleChange = (event) => {
    setValue(event.target.value);
	  invoke('save_api_key', {key: event.target.value})
    onChange(event.target.value);
  };

  return (
    <div className="text-input">
	  <input
        type="text"
        value={value}
        onChange={handleChange}
        placeholder="Enter API key ..."
      />
    </div>
  );
};

export default ApiTextInput;

