export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'createCommunity' : IDL.Func(
        [
          IDL.Text,
          IDL.Text,
          IDL.Vec(
            IDL.Record({
              'principal' : IDL.Principal,
              'username' : IDL.Text,
              'profileImage' : IDL.Vec(IDL.Nat8),
            })
          ),
        ],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'members' : IDL.Vec(
                IDL.Record({
                  'principal' : IDL.Principal,
                  'username' : IDL.Text,
                  'profileImage' : IDL.Vec(IDL.Nat8),
                })
              ),
              'name' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'createCourse' : IDL.Func(
        [IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'createNewsArticle' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Nat8), IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'content' : IDL.Text,
              'date' : IDL.Text,
              'author' : IDL.Text,
              'image' : IDL.Vec(IDL.Nat8),
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'deleteCourse' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'deleteNewsArticle' : IDL.Func(
        [IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'content' : IDL.Text,
              'date' : IDL.Text,
              'author' : IDL.Text,
              'image' : IDL.Vec(IDL.Nat8),
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'getCommunityByID' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'members' : IDL.Vec(
                IDL.Record({
                  'principal' : IDL.Principal,
                  'username' : IDL.Text,
                  'profileImage' : IDL.Vec(IDL.Nat8),
                })
              ),
              'name' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'getCommunityMembers' : IDL.Func(
        [IDL.Text],
        [
          IDL.Vec(
            IDL.Record({
              'principal' : IDL.Principal,
              'username' : IDL.Text,
              'profileImage' : IDL.Vec(IDL.Nat8),
            })
          ),
        ],
        ['query'],
      ),
    'getCourseById' : IDL.Func(
        [IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'getNewsArticleById' : IDL.Func(
        [IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'content' : IDL.Text,
              'date' : IDL.Text,
              'author' : IDL.Text,
              'image' : IDL.Vec(IDL.Nat8),
            }),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'joinCommunity' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Principal],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'members' : IDL.Vec(
                IDL.Record({
                  'principal' : IDL.Principal,
                  'username' : IDL.Text,
                  'profileImage' : IDL.Vec(IDL.Nat8),
                })
              ),
              'name' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'leaveCommunity' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'members' : IDL.Vec(
                IDL.Record({
                  'principal' : IDL.Principal,
                  'username' : IDL.Text,
                  'profileImage' : IDL.Vec(IDL.Nat8),
                })
              ),
              'name' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'listCourses' : IDL.Func(
        [],
        [
          IDL.Variant({
            'Ok' : IDL.Vec(
              IDL.Record({
                'id' : IDL.Principal,
                'title' : IDL.Text,
                'description' : IDL.Text,
              })
            ),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'listNewsArticles' : IDL.Func(
        [],
        [
          IDL.Variant({
            'Ok' : IDL.Vec(
              IDL.Record({
                'id' : IDL.Principal,
                'title' : IDL.Text,
                'content' : IDL.Text,
                'date' : IDL.Text,
                'author' : IDL.Text,
                'image' : IDL.Vec(IDL.Nat8),
              })
            ),
            'Err' : IDL.Text,
          }),
        ],
        ['query'],
      ),
    'updateCourse' : IDL.Func(
        [IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'description' : IDL.Text,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'updateNewsArticle' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Nat8), IDL.Text, IDL.Text],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Principal,
              'title' : IDL.Text,
              'content' : IDL.Text,
              'date' : IDL.Text,
              'author' : IDL.Text,
              'image' : IDL.Vec(IDL.Nat8),
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
