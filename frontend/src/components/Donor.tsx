import React from 'react';

interface DonorProps {
    account: string;
    profileImage: string;
    charitiesDonatedTo: string[];
}

const Donator: React.FC<DonorProps> = ({ account, profileImage, charitiesDonatedTo }) => {
    return (
        <div className="donator">
            <h2>Donator Account</h2>
            <img src={profileImage} alt="Profile" />
            <p>{account}</p>
            <h3>Charities Donated To:</h3>
            <ul>
                {charitiesDonatedTo.map((charity, index) => (
                    <li key={index}>{charity}</li>
                ))}
            </ul>
        </div>
    );
};

export default Donator;

