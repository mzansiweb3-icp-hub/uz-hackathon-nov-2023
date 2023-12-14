import {
  nat,
  int,
  text,
  nat64,
  Record,
} from "azle";

export const ArtistId = text;
export const UserId = text;
export const ManagerId = text;
export const LogsheetId = text;
export const SongId = text;
  
export const Artist = Record({
   id:text,
   FullName:text,
   Pseudoname:text,
   National_ID:text,
   Date_of_Birth:text,
   Nationality:text,
   Genre: text,
   Phonenumber:text,
   Email: text,
});
  
  export type Artist = typeof Artist;

  export const ArtistPayload = Record({

    FullName:text,
    Pseudoname:text,
    National_ID:text,
    Date_of_Birth:text,
    Nationality:text,
    Genre: text,
    Phonenumber:text,
    Email: text,
 
   });
   
  export type ArtistPayload = typeof ArtistPayload;

  export const User = Record({
    
    id:text,
    Name:text,
    Phonenumber:text,
    Email: text,
 
   });
   export type User = typeof User;

   export const UserPayload = Record({
    
    Name:text,
    Phonenumber:text,
    Email: text,
 
   });

   export type UserPayload = typeof UserPayload;

   export const Manager = Record({
    
    id:text,
    Name:text,
    Phonenumber:text,
    Email: text,
 
   });

   export type Manager = typeof Manager;

   export const ManagerPayload = Record({
    
    Name:text,
    Phonenumber:text,
    Email: text,
 
   });

   export type ManagerPayload = typeof ManagerPayload;


   export const Song = Record({
    
    id:text,
    Title:text,
    Composer:text,
    ArtistId: text,
    PlayCount: int,
 
   });

   export type Song = typeof Song;
   export const SongPayload = Record({
    
    Title:text,
    Composer:text,
    ArtistId: text,
    PlayCount: int,
 
   });

   export type SongPayload = typeof SongPayload;

   export const Logsheet = Record({

    id:text,
    createdBy:text,
    SongTitle:text,
    NumberOfPlays:int,
    Composer: text,
  });
  export type Logsheet = typeof Logsheet;

  export const LogsheetPayload = Record({
    createdBy:text,
    SongTitle:text,
    NumberOfPlays:int,
    Composer: text,
  });
  export type LogsheetPayload = typeof LogsheetPayload;




   
   

