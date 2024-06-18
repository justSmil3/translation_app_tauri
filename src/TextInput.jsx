import React, { useState } from 'react';
import './TextInput.css';

const TextInput = ({onChange }) => {
  const [value, setValue] = useState('');

  const handleChange = (event) => {
    setValue(event.target.value);
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

export default TextInput;

