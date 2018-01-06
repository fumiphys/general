var gulp = require('gulp');
var typescript = require('gulp-typescript');

gulp.task('compile', function(){
  return gulp.src(['./*.ts'])
  .pipe(typescript())
  .js
  .pipe(gulp.dest('./'));
});

gulp.task('default', ['compile']);
