import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'createCommunity' : ActorMethod<
    [
      string,
      string,
      Array<
        {
          'principal' : Principal,
          'username' : string,
          'profileImage' : Uint8Array | number[],
        }
      >,
    ],
    {
        'Ok' : {
          'id' : Principal,
          'members' : Array<
            {
              'principal' : Principal,
              'username' : string,
              'profileImage' : Uint8Array | number[],
            }
          >,
          'name' : string,
          'description' : string,
        }
      } |
      { 'Err' : string }
  >,
  'createCourse' : ActorMethod<
    [string, string],
    { 'Ok' : { 'id' : Principal, 'title' : string, 'description' : string } } |
      { 'Err' : string }
  >,
  'createNewsArticle' : ActorMethod<
    [string, string, Uint8Array | number[], string, string],
    {
        'Ok' : {
          'id' : Principal,
          'title' : string,
          'content' : string,
          'date' : string,
          'author' : string,
          'image' : Uint8Array | number[],
        }
      } |
      { 'Err' : string }
  >,
  'deleteCourse' : ActorMethod<
    [string],
    { 'Ok' : { 'id' : Principal, 'title' : string, 'description' : string } } |
      { 'Err' : string }
  >,
  'deleteNewsArticle' : ActorMethod<
    [string, string],
    {
        'Ok' : {
          'id' : Principal,
          'title' : string,
          'content' : string,
          'date' : string,
          'author' : string,
          'image' : Uint8Array | number[],
        }
      } |
      { 'Err' : string }
  >,
  'getCommunityByID' : ActorMethod<
    [string],
    {
        'Ok' : {
          'id' : Principal,
          'members' : Array<
            {
              'principal' : Principal,
              'username' : string,
              'profileImage' : Uint8Array | number[],
            }
          >,
          'name' : string,
          'description' : string,
        }
      } |
      { 'Err' : string }
  >,
  'getCommunityMembers' : ActorMethod<
    [string],
    Array<
      {
        'principal' : Principal,
        'username' : string,
        'profileImage' : Uint8Array | number[],
      }
    >
  >,
  'getCourseById' : ActorMethod<
    [string],
    { 'Ok' : { 'id' : Principal, 'title' : string, 'description' : string } } |
      { 'Err' : string }
  >,
  'getNewsArticleById' : ActorMethod<
    [string, string],
    {
        'Ok' : {
          'id' : Principal,
          'title' : string,
          'content' : string,
          'date' : string,
          'author' : string,
          'image' : Uint8Array | number[],
        }
      } |
      { 'Err' : string }
  >,
  'joinCommunity' : ActorMethod<
    [string, string, Principal],
    {
        'Ok' : {
          'id' : Principal,
          'members' : Array<
            {
              'principal' : Principal,
              'username' : string,
              'profileImage' : Uint8Array | number[],
            }
          >,
          'name' : string,
          'description' : string,
        }
      } |
      { 'Err' : string }
  >,
  'leaveCommunity' : ActorMethod<
    [string, Principal],
    {
        'Ok' : {
          'id' : Principal,
          'members' : Array<
            {
              'principal' : Principal,
              'username' : string,
              'profileImage' : Uint8Array | number[],
            }
          >,
          'name' : string,
          'description' : string,
        }
      } |
      { 'Err' : string }
  >,
  'listCourses' : ActorMethod<
    [],
    {
        'Ok' : Array<
          { 'id' : Principal, 'title' : string, 'description' : string }
        >
      } |
      { 'Err' : string }
  >,
  'listNewsArticles' : ActorMethod<
    [],
    {
        'Ok' : Array<
          {
            'id' : Principal,
            'title' : string,
            'content' : string,
            'date' : string,
            'author' : string,
            'image' : Uint8Array | number[],
          }
        >
      } |
      { 'Err' : string }
  >,
  'updateCourse' : ActorMethod<
    [string, string],
    { 'Ok' : { 'id' : Principal, 'title' : string, 'description' : string } } |
      { 'Err' : string }
  >,
  'updateNewsArticle' : ActorMethod<
    [string, string, Uint8Array | number[], string, string],
    {
        'Ok' : {
          'id' : Principal,
          'title' : string,
          'content' : string,
          'date' : string,
          'author' : string,
          'image' : Uint8Array | number[],
        }
      } |
      { 'Err' : string }
  >,
}
