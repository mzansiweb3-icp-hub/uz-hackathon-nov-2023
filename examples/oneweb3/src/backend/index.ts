import {
    Canister,
    ic,
    Err,
    nat64,
    Ok,
    Opt,
    Principal,
    query,
    Record,
    Result,
    StableBTreeMap,
    text,
    update,
    Variant,
    Vec,
    blob
} from 'azle';

import { v4 as uuidv4 } from "uuid";

const User = Record({
  principal: Principal,
  username: text,
  profileImage: blob,
});

type User = typeof User;

const NewsArticle = Record({
  id: Principal,
  title: text,
  content: text,
  image: blob,
  date: text,
  author: text,
});

type NewsArticle = typeof NewsArticle;

const Community = Record({
  id: Principal,
  name: text,
  description: text,
  members: Vec(User),
});

type Community = typeof Community;

const Course = Record({
  id: Principal,
  title: text,
  description: text,
});

type Course = typeof Course;


let userDataStorage = StableBTreeMap(Principal, User, 0);

let newsArticleStorage = StableBTreeMap(Principal, NewsArticle, 1);

let communityStorage = StableBTreeMap(Principal, Community, 2);

let courseStorage = StableBTreeMap(Principal, Course, 3);


export default Canister({
  createNewsArticle: update([text, text, blob, text, text], Result(NewsArticle, text), (title, content, image, date, author) => {

      if (!title || !content || !image || !date || !author) {
          return Result.Err('Please provide all required information.');
      }

      // Generate a unique ID for the news article
      const id = Principal.fromText(`${title}-${date}`);
      const newsArticle: NewsArticle = { id, title, content, image, date, author };

      newsArticleStorage.insert(id, newsArticle);

      return Result.Ok(newsArticle);
  }),

    getNewsArticle: query([text, text], Variant({ Ok: Vec(NewsArticle), Err: text }), (title, date) => {
        // Validate input parameters
        
        const id = Principal.fromText(`${title}-${Date}`);
        const newsArticleOpt = newsArticleStorage.get(id);

        if ('Some' in newsArticleOpt) {
            return { Ok: newsArticleOpt.Some };
        } else {
            return { Err: 'News article not found.' };
        }
    }),

  listNewsArticles: query([], Variant({ Ok: Vec(NewsArticle), Err: text }), () => {
    // Get all news articles
    const newsArticles = newsArticleStorage.entries().map((entry) => entry[1]);

    return { Ok: newsArticles };
  }),

  updateNewsArticle: update([text, text, blob, text, text], Result(NewsArticle, text), (title, content, image, date, author) => {
    if (!title || !content || !image || !date || !author) {
        return Result.Err('Invalid input parameters. Please provide all required information.');
    }
    const id = Principal.fromText(`${title}-${date}`);
    const newsArticleOpt = newsArticleStorage.get(id);

    if ('Some' in newsArticleOpt) {
        const updatedNewsArticle: NewsArticle = { ...newsArticleOpt.Some, title, content, image, author };

        // Update the news article
        newsArticleStorage.insert(id, updatedNewsArticle);

        return Result.Ok(updatedNewsArticle);
    } else {
        return Result.Err('News article not found.');
    }
  }),

  deleteNewsArticle: update([text, text], Result(NewsArticle, text), (title, date) => {
    if (!title || !date) {
        return Result.Err('Please provide title and date.');
    }
    const id = Principal.fromText(`${title}-${date}`);
    const newsArticleOpt = newsArticleStorage.get(id);

    if ('Some' in newsArticleOpt) {
        // Remove the news article
        newsArticleStorage.delete(id);

        return Result.Ok(newsArticleOpt.Some);
    } else {
        return Result.Err('News article not found.');
    }
  }),

   // Create a new community
   createCommunity: update([text, text, Vec(User)], Result(Community, text), (name, description, members) => {
    if (!name || !description || !members) {
        return Result.Err('Invalid input parameters. Please provide all required information.');
    }

    // Generate a unique ID for the community
    const id = Principal.fromText(`${name}-${Date.now()}`);
    const community: Community = { id, name, description, members };

    // Initialize community data
    communityStorage.insert(id, community);

    return Result.Ok(community);
  }),

// Get community details by ID
  getCommunityById: query([text], Variant({ Ok: Community, Err: text }), (id) => {


      // Get the community by ID
      const communityOpt = communityStorage.get(id);

      if ('Some' in communityOpt) {
          return { Ok: communityOpt.Some };
      } else {
          return { Err: 'Community not found.' };
      }
  }),

  // Join a community
  joinCommunity: update([text, text, Principal], Result(Community, text), (communityId, username, memberPrincipal) => {

      if (!communityId || !username || !memberPrincipal) {
          return Result.Err('Invalid input parameters. Please provide all required information.');
      }

      // Get the community by ID
      const communityOpt = communityStorage.get(communityId);

      if ('Some' in communityOpt) {
          const community = communityOpt.Some;

          // Add the member to the community
          const updatedMembers = community.members.concat({ principal: memberPrincipal, username });

          // Update the community
          communityStorage.insert(communityId, { ...community, members: updatedMembers });

          return Result.Ok({ ...community, members: updatedMembers });
      } else {
          return Result.Err('Community not found.');
      }
  }),

  // Leave a community
  leaveCommunity: update([text, Principal], Result(Community, text), (communityId, memberPrincipal) => {

      if (!communityId || !memberPrincipal) {
          return Result.Err('Please provide all required information.');
      }

      // Get the community by ID
      const communityOpt = communityStorage.get(communityId);

      if ('Some' in communityOpt) {
          const community = communityOpt.Some;

          // Remove the member from the community
          const updatedMembers = community.members.filter(member => member.principal !== memberPrincipal);

          // Update the community
          communityStorage.insert(communityId, { ...community, members: updatedMembers });

          return Result.Ok({ ...community, members: updatedMembers });
      } else {
          return Result.Err('Community not found.');
      }
  }),

  // Get a list of members in a community
  getCommunityMembers: query([text], Variant({ Ok: Vec(User), Err: text }), (communityId) => {

      if (!communityId) {
          return { Err: 'Please provide the community ID.' };
      }

      // Get the community by ID
      const communityOpt = communityStorage.get(communityId);

      if ('Some' in communityOpt) {
          const community = communityOpt.Some;

          // Return the list of members in the community
          return { Ok: community.members };
      } else {
          return { Err: 'Community not found.' };
      }
  }),


  createCourse: update([text, text], Result(Course, text), (title, description) => {
    if (!title || !description) {
        return Result.Err('Invalid input parameters. Please provide all required information.');
    }

    // Generate ID for the course
    const id = Principal.fromText(`${title}-${Date.now()}`);
    const course: Course = { id, title, description };

    // Initialize course data
    courseStorage.insert(id, course);

    return Result.Ok(course);
  }),

  getCourseById: query([Principal], Variant({ Ok: Course, Err: text }), (id) => {
    if (!id) {
        return { Err: 'Please provide the course ID.' };
    }

    const courseOpt = courseStorage.get(id);

    if ('Some' in courseOpt) {
        return { Ok: courseOpt.Some };
    } else {
        return { Err: 'Course not found.' };
    }
  }),

  listCourses: query([], Variant({ Ok: Vec(Course), Err: text }), () => {
    // Get all courses
    const courses = courseStorage.entries().map((entry) => entry[1]);

    return { Ok: courses };
  }),

  updateCourse: update([text, text], Result(Course, text), (id, newDescription) => {
    if (!id || !newDescription) {
        return Result.Err('Invalid input parameters. Please provide all required information.');
    }

    const courseOpt = courseStorage.get(id);

    if ('Some' in courseOpt) {
        const course = courseOpt.Some;

        const updatedCourse: Course = { ...course, description: newDescription };

        courseStorage.insert(id, updatedCourse);

        return Result.Ok(updatedCourse);
    } else {
        return Result.Err('Course not found.');
    }
  }),

  // Delete a course
  deleteCourse: update([text], Result(Course, text), (id) => {
    if (!id) {
        return Result.Err('Invalid input parameters. Please provide the course ID.');
    }

    const courseOpt = courseStorage.get(id);

    if ('Some' in courseOpt) {
    
        courseStorage.delete(id);

        return Result.Ok(courseOpt.Some);
    } else {
        return Result.Err('Course not found.');
    }
  }),


});


globalThis.crypto = {
  // @ts-ignore
 getRandomValues: () => {
     let array = new Uint8Array(32)

     for (let i = 0; i < array.length; i++) {
         array[i] = Math.floor(Math.random() * 256)
     }

     return array
 }
}

