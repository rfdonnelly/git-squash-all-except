# frozen_string_literal: true

require 'open3'

directory 'tush' do
  sh 'git clone https://github.com/adolfopa/tush'
  sh 'git -C tush checkout -q af3dd4a0813d65e68d4c4a12b2aa31ba9d07c2a5'

  # This patch fixes a syntax error with newer versions of awk
  patch = <<~'EOF'
    diff --git a/bin/tush-expand b/bin/tush-expand
    index 4fd2d1c..1e3bedb 100755
    --- a/bin/tush-expand
    +++ b/bin/tush-expand
    @@ -5,7 +5,7 @@
     	expand_variable(0)
     }
     
    -/^(\||\@).*\$\{[[:alnum:]_]+\}/ {
    +/^(\||@).*\$\{[[:alnum:]_]+\}/ {
         while (match($0, /\$\{[[:alnum:]_]+\}/))
     	expand_variable(1)
     }
  EOF

  stdout, status = Open3.capture2('git -C tush apply --ignore-whitespace', stdin_data: patch)
  abort 'unable to git apply patch' unless status.success?
end

TUSH_PATH = File.join(__dir__, %w[tush bin])
BIN_PATH = File.join(__dir__, 'shell')
TEST_PATH = File.join(__dir__, %w[tests])
ENV['PATH'] = [
  TUSH_PATH,
  BIN_PATH,
  TEST_PATH,
  ENV['PATH'],
].join(':')
ROOT_PATH = File.dirname(File.expand_path(__FILE__))
TESTS = FileList['tests/*_test.adoc']
TESTS.each do |test|
  task test => 'tush' do
    sh "ROOT_PATH=#{ROOT_PATH} runtest #{test}"
  end
end

multitask test: TESTS

multitask default: :test
