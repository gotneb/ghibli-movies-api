# About

> The unofficial and small Studio Ghibli api.

It's `small` cause I don't plan it to have all Studio Ghibli details like world, vehicles, characters, etc... Only the movies.

![Banner Repository](https://github.com/gotneb/ghibli-movies-api/blob/master/repo_banner.png)

⚠️ I use a **free plan** for the hosting, so the server sleeps after **15 minutes of inactive.** So only the first request takes a while...

## Routes

### Path: https://studio-ghibli-movies-api.onrender.com

### Search (GET)
Searches a movie by its title. Returns a list of all results found.
```
/movies/search/:searchTerm
```
<details>

<summary>Example</summary>

```
https://studio-ghibli-movies-api.onrender.com/movies/search/whisper
```

```json
{
  "id": "75d02d59b6679afa885edced77f2b6131dc8039c6dd3927bf6537226094808e4",
  "title": "Whisper of Hearts",
  "original_title": "耳をすませば",
  "alternative_title": "Mimi wo Sumaseba",
  "poster_title": "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/LmEnxtiAuzezXBjYXPuDgfZ4zZQ/AAAABWWyvMGriEaB0oV3gVDqqsKI7-I5VmkJXqbF0LwN1_f003lJFZmTAdaYJxcXDTKp-rKaEptUNgvLckaPC9S259j5yI-gVhlfJu3yZ4_vZcAy.png?r=9dc",
  "poster": "https://www.ghibli.jp/images/mimi.jpg",
  "description": "Shizuku lives a simple life, dominated by her love for stories and writing. One day she notices that all the library books she has have been previously checked out by the same person: 'Seiji Amasawa'. Curious as to who he is, Shizuku meets a boy her age whom she finds infuriating, but discovers to her shock that he is her 'Prince of Books'. As she grows closer to him, she realises that he merely read all those books to bring himself closer to her. The boy Seiji aspires to be a violin maker in Italy, and it is his dreams that make Shizuku realise that she has no clear path for her life. Knowing that her strength lies in writing, she tests her talents by writing a story about Baron, a cat statuette belonging to Seiji's grandfather.",
  "background_poster": "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/6AYY37jfdO6hpXcMjf9Yu5cnmO0/AAAABXntjPKOPRVT7uCCqL-wQz044AUIGHkXF_NhP3aUS1opgQfISHd4ok_o88_WVizcM64PSG6dVkDDVEnufu-0s34jQVVz7H_YimzI.jpg?r=26f",
  "director": "Hayao Miyazaki",
  "release_year": 1995,
  "duration": 111,
  "score": 8.2,
  "trailer": "https://youtu.be/0pVkiod6V0U",
  "genres": [
    "Romance",
    "Drama",
    "Adventure"
  ],
  "gallery": [
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRYtEH1UfmEUVaYbEh8ygopWfF-HSrBCHsjjz6qnmOEQl9D8nxeWyptFdrP8Nb2LcS3kmg&usqp=CAU",
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRP-g9aTDoFW3YmMe_MCRjhYnFt44y_djlXNpuP3P0hu4wd1cIT0qs1EX0QlwNYf2rVdbM&usqp=CAU",
    "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/E8vDc_W8CLv7-yMQu8KMEC7Rrr8/AAAABZNhSHL7fd8rzGlIeydmhtXDxKyXx9J-Bi_m1jKQc2pjhYjfoPc4-sbh0-MtxtVhemjhOmFasw8QILbKiowBVoyJgJ7EyOpxwiEk.jpg?r=cb9",
    "https://thequotorium.files.wordpress.com/2020/08/screenshot-2020-08-05-12.31.55-e1596645153507.png?w=613&h=329",
    "https://catsonfilm.files.wordpress.com/2013/01/whispers02.jpg",
    "https://thequotorium.files.wordpress.com/2020/07/screenshot-2020-07-14-18.41.45-e1594766592196.png?w=1200",
    "https://3.bp.blogspot.com/-AuR3SR2rCP8/TewzPlFeIuI/AAAAAAAAALc/zJ8TrXZw92E/s1600/landscape.png",
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRttraf2meMnY0w96yutrxVpgBVXykUz4zPe4ONMAVUVhuTX5vDKi1ZOk3GO-GxKUzQJ2w&usqp=CAU",
    "https://sendauponatime.files.wordpress.com/2013/11/whisper-of-the-heart-book-cards1.jpg"
  ]
}
```

</details>

### Find (GET)
Finds a specific movie by its `id`.
```
/movies/get/:id
```
<details>

<summary>Example</summary>

```
https://studio-ghibli-movies-api.onrender.com/movies/get/75d02d59b6679afa885edced77f2b6131dc8039c6dd3927bf6537226094808e4
```

```json
[
  {
    "id": "75d02d59b6679afa885edced77f2b6131dc8039c6dd3927bf6537226094808e4",
    "title": "Whisper of Hearts",
    "original_title": "耳をすませば",
    "alternative_title": "Mimi wo Sumaseba",
    "poster_title": "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/LmEnxtiAuzezXBjYXPuDgfZ4zZQ/AAAABWWyvMGriEaB0oV3gVDqqsKI7-I5VmkJXqbF0LwN1_f003lJFZmTAdaYJxcXDTKp-rKaEptUNgvLckaPC9S259j5yI-gVhlfJu3yZ4_vZcAy.png?r=9dc",
    "poster": "https://www.ghibli.jp/images/mimi.jpg",
    "description": "Shizuku lives a simple life, dominated by her love for stories and writing. One day she notices that all the library books she has have been previously checked out by the same person: 'Seiji Amasawa'. Curious as to who he is, Shizuku meets a boy her age whom she finds infuriating, but discovers to her shock that he is her 'Prince of Books'. As she grows closer to him, she realises that he merely read all those books to bring himself closer to her. The boy Seiji aspires to be a violin maker in Italy, and it is his dreams that make Shizuku realise that she has no clear path for her life. Knowing that her strength lies in writing, she tests her talents by writing a story about Baron, a cat statuette belonging to Seiji's grandfather.",
    "background_poster": "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/6AYY37jfdO6hpXcMjf9Yu5cnmO0/AAAABXntjPKOPRVT7uCCqL-wQz044AUIGHkXF_NhP3aUS1opgQfISHd4ok_o88_WVizcM64PSG6dVkDDVEnufu-0s34jQVVz7H_YimzI.jpg?r=26f",
    "director": "Hayao Miyazaki",
    "release_year": 1995,
    "duration": 111,
    "score": 8.2,
    "trailer": "https://youtu.be/0pVkiod6V0U",
    "genres": [
      "Romance",
      "Drama",
      "Adventure"
    ],
    "gallery": [
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRYtEH1UfmEUVaYbEh8ygopWfF-HSrBCHsjjz6qnmOEQl9D8nxeWyptFdrP8Nb2LcS3kmg&usqp=CAU",
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRP-g9aTDoFW3YmMe_MCRjhYnFt44y_djlXNpuP3P0hu4wd1cIT0qs1EX0QlwNYf2rVdbM&usqp=CAU",
      "https://occ-0-2794-2219.1.nflxso.net/dnm/api/v6/E8vDc_W8CLv7-yMQu8KMEC7Rrr8/AAAABZNhSHL7fd8rzGlIeydmhtXDxKyXx9J-Bi_m1jKQc2pjhYjfoPc4-sbh0-MtxtVhemjhOmFasw8QILbKiowBVoyJgJ7EyOpxwiEk.jpg?r=cb9",
      "https://thequotorium.files.wordpress.com/2020/08/screenshot-2020-08-05-12.31.55-e1596645153507.png?w=613&h=329",
      "https://catsonfilm.files.wordpress.com/2013/01/whispers02.jpg",
      "https://thequotorium.files.wordpress.com/2020/07/screenshot-2020-07-14-18.41.45-e1594766592196.png?w=1200",
      "https://3.bp.blogspot.com/-AuR3SR2rCP8/TewzPlFeIuI/AAAAAAAAALc/zJ8TrXZw92E/s1600/landscape.png",
      "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRttraf2meMnY0w96yutrxVpgBVXykUz4zPe4ONMAVUVhuTX5vDKi1ZOk3GO-GxKUzQJ2w&usqp=CAU",
      "https://sendauponatime.files.wordpress.com/2013/11/whisper-of-the-heart-book-cards1.jpg"
    ]
  }
]
```

</details>

### All (GET)
Returns a list of all movies.
```
/movies/all
```
### Random (GET)
Randomly returns a movie.
```
/movies/random
```

### Motivation
I'm studying the Rust programming language, and now I'm confident that I can build something simple, but reliable to commmunity and also, blaaazingly fast 🚀.

> The other reason is that a close friend of mine loves Ghibli movies, and she's studying web dev, hence I had the ideia to make a basic API about Ghibli. Hope one day she'll find out about this repo.

### Todo
- [ ] Add all movies

