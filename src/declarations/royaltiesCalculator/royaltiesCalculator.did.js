export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'addArtist' : IDL.Func(
        [
          IDL.Record({
            'Email' : IDL.Text,
            'Genre' : IDL.Text,
            'FullName' : IDL.Text,
            'National_ID' : IDL.Text,
            'Nationality' : IDL.Text,
            'Phonenumber' : IDL.Text,
            'Pseudoname' : IDL.Text,
            'Date_of_Birth' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'Email' : IDL.Text,
            'Genre' : IDL.Text,
            'FullName' : IDL.Text,
            'National_ID' : IDL.Text,
            'Nationality' : IDL.Text,
            'Phonenumber' : IDL.Text,
            'Pseudoname' : IDL.Text,
            'Date_of_Birth' : IDL.Text,
          }),
        ],
        [],
      ),
    'addDummyData' : IDL.Func([], [IDL.Text], []),
    'addLogsheet' : IDL.Func(
        [
          IDL.Record({
            'createdBy' : IDL.Text,
            'Composer' : IDL.Text,
            'NumberOfPlays' : IDL.Int,
            'SongTitle' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'createdBy' : IDL.Text,
            'Composer' : IDL.Text,
            'NumberOfPlays' : IDL.Int,
            'SongTitle' : IDL.Text,
          }),
        ],
        [],
      ),
    'addManager' : IDL.Func(
        [
          IDL.Record({
            'Email' : IDL.Text,
            'Name' : IDL.Text,
            'Phonenumber' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'Email' : IDL.Text,
            'Name' : IDL.Text,
            'Phonenumber' : IDL.Text,
          }),
        ],
        [],
      ),
    'addSong' : IDL.Func(
        [
          IDL.Record({
            'PlayCount' : IDL.Int,
            'ArtistId' : IDL.Text,
            'Composer' : IDL.Text,
            'Title' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'PlayCount' : IDL.Int,
            'ArtistId' : IDL.Text,
            'Composer' : IDL.Text,
            'Title' : IDL.Text,
          }),
        ],
        [],
      ),
    'addUser' : IDL.Func(
        [
          IDL.Record({
            'Email' : IDL.Text,
            'Name' : IDL.Text,
            'Phonenumber' : IDL.Text,
          }),
        ],
        [
          IDL.Record({
            'id' : IDL.Text,
            'Email' : IDL.Text,
            'Name' : IDL.Text,
            'Phonenumber' : IDL.Text,
          }),
        ],
        [],
      ),
    'calculateRoyalties' : IDL.Func([IDL.Nat], [IDL.Vec(IDL.Text)], ['query']),
    'getAllSongs' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Record({
              'id' : IDL.Text,
              'PlayCount' : IDL.Int,
              'ArtistId' : IDL.Text,
              'Composer' : IDL.Text,
              'Title' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
    'getAllUsers' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Record({
              'id' : IDL.Text,
              'Email' : IDL.Text,
              'Genre' : IDL.Text,
              'FullName' : IDL.Text,
              'National_ID' : IDL.Text,
              'Nationality' : IDL.Text,
              'Phonenumber' : IDL.Text,
              'Pseudoname' : IDL.Text,
              'Date_of_Birth' : IDL.Text,
            })
          ),
        ],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
