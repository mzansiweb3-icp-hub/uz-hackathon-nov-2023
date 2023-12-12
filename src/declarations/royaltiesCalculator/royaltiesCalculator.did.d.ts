import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'addArtist' : ActorMethod<
    [
      {
        'Email' : string,
        'Genre' : string,
        'FullName' : string,
        'National_ID' : string,
        'Nationality' : string,
        'Phonenumber' : string,
        'Pseudoname' : string,
        'Date_of_Birth' : string,
      },
    ],
    {
      'id' : string,
      'Email' : string,
      'Genre' : string,
      'FullName' : string,
      'National_ID' : string,
      'Nationality' : string,
      'Phonenumber' : string,
      'Pseudoname' : string,
      'Date_of_Birth' : string,
    }
  >,
  'addDummyData' : ActorMethod<[], string>,
  'addLogsheet' : ActorMethod<
    [
      {
        'createdBy' : string,
        'Composer' : string,
        'NumberOfPlays' : bigint,
        'SongTitle' : string,
      },
    ],
    {
      'id' : string,
      'createdBy' : string,
      'Composer' : string,
      'NumberOfPlays' : bigint,
      'SongTitle' : string,
    }
  >,
  'addManager' : ActorMethod<
    [{ 'Email' : string, 'Name' : string, 'Phonenumber' : string }],
    { 'id' : string, 'Email' : string, 'Name' : string, 'Phonenumber' : string }
  >,
  'addSong' : ActorMethod<
    [
      {
        'PlayCount' : bigint,
        'ArtistId' : string,
        'Composer' : string,
        'Title' : string,
      },
    ],
    {
      'id' : string,
      'PlayCount' : bigint,
      'ArtistId' : string,
      'Composer' : string,
      'Title' : string,
    }
  >,
  'addUser' : ActorMethod<
    [{ 'Email' : string, 'Name' : string, 'Phonenumber' : string }],
    { 'id' : string, 'Email' : string, 'Name' : string, 'Phonenumber' : string }
  >,
  'calculateRoyalties' : ActorMethod<[bigint], Array<string>>,
  'getAllSongs' : ActorMethod<
    [],
    Array<
      {
        'id' : string,
        'PlayCount' : bigint,
        'ArtistId' : string,
        'Composer' : string,
        'Title' : string,
      }
    >
  >,
  'getAllUsers' : ActorMethod<
    [],
    Array<
      {
        'id' : string,
        'Email' : string,
        'Genre' : string,
        'FullName' : string,
        'National_ID' : string,
        'Nationality' : string,
        'Phonenumber' : string,
        'Pseudoname' : string,
        'Date_of_Birth' : string,
      }
    >
  >,
}
