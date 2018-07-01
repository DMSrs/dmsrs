'use strict';
const gulp = require('gulp');
const less = require('gulp-less');
const cssnano = require('gulp-cssnano');
const del = require('del');

gulp.task('css', (done) => {
    gulp.src('src/less/*.less')
        .pipe(less())
        .pipe(cssnano())
        .pipe(gulp.dest('static/css'));
    done();
});

gulp.task('clean', (done) => {
    del.sync('static/css');
    done();
});

gulp.task('watch', () => {
    return gulp.watch('src/less/*', gulp.series(['css']));
});

gulp.task('build', gulp.series(['css']), ()=>{});
gulp.task('cleanandbuild', gulp.series(['clean', 'build']), ()=>{});
gulp.task('default', gulp.series(['cleanandbuild']));

