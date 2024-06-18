import React, { useState, useEffect } from 'react';
import './MultiDropdown.css';

const MultiDropdown = ({ options, onSelect }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedOptions, setSelectedOptions] = useState([options[1]]);
  

  const toggleDropdown = () => setIsOpen(!isOpen);

	useEffect(()=>{
		let newSelectedOptions = selectedOptions.filter(o => options.includes(o));
		setSelectedOptions(newSelectedOptions);
		onSelect(newSelectedOptions);
	},[options]);

  const handleOptionClick = (option) => {
    let newSelectedOptions;
    if (selectedOptions.includes(option)) {
      newSelectedOptions = selectedOptions.filter(item => item !== option);
    } else {
      newSelectedOptions = [...selectedOptions, option];
    }
    setSelectedOptions(newSelectedOptions);
    onSelect(newSelectedOptions);
  };

	const handleOptionAllClick = () => {
		if (selectedOptions.length === options.length){
			setSelectedOptions([]);
			onSelect([])
		} else{
			setSelectedOptions(options);
			onSelect(options);
		}
	}

  return (
    <div className="multidropdown">
      <button className="multidropdown-toggle" onClick={toggleDropdown}>
        {selectedOptions.length > 0 ? selectedOptions.join(', ') : 'Select options'}
      </button>
      {isOpen && (
        <ul className="multidropdown-menu">
          {options.map((option, index) => (
            <li
              key={index}
              className={`multidropdown-item ${selectedOptions.includes(option) ? 'selected' : ''}`}
              onClick={() => handleOptionClick(option)}
            >
              {option}
            </li>
          ))}
		  <li
			className={`multidropdown-item ${(selectedOptions.length === options.length) ? 'selected': ''}`}
			onClick={()=>handleOptionAllClick()}>
			All
		  </li>
        </ul>
      )}
    </div>
  );
};

export default MultiDropdown;

