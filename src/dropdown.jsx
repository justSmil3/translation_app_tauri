import React, { useState } from 'react';
import './Dropdown.css'; 
const Dropdown = ({ options, onSelect }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [selectedOption, setSelectedOption] = useState(options[0]);

  const toggleDropdown = () => setIsOpen(!isOpen);
  
  const handleOptionClick = (option) => {
	  if (selectedOption == option){
		  setSelectedOption("Auto Detect");
		  onSelect("Auto Detect");
	  }
	  else{
		setSelectedOption(option);
		onSelect(option);
	  }
	  setIsOpen(false);
  };

  return (
    <div className="dropdown">
      <button className="dropdown-toggle" onClick={toggleDropdown}>
        {selectedOption || 'Select an option'}
      </button>
      {isOpen && (
        <ul className="dropdown-menu">
          {options.map((option, index) => (
            <li
              key={index}
              className="dropdown-item"
              onClick={() => handleOptionClick(option)}
            >
              {option}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default Dropdown;

