import React from 'react';

interface CharityProps {
    account: string;
    profileImage: string;
    donators: string[];
}

const Charity: React.FC<CharityProps> = ({ account, profileImage, donators }) => {
    return (
<Card>
        className="charity">
            <h2>Cancer Research UK</h2>
            <img src={profileImage} alt="Profile" />
            <p>{account}</p>
            <h3>Donators:</h3>
            <ul>
                {donators.map((donator, index) => (
                    <li key={index}>{donator}</li>
                ))}
            </ul>
        </div>
    );
};