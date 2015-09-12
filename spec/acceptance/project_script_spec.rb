require_relative '../../spec/spec_helper'

describe 'project', type: :feature do

  before do
    configure do
      { observed_folders: [] }
    end
  end

  describe 'wrong arguments' do
    subject { run("project something") }

    it { is_expected.to include "Invalid arguments." }

    it 'returns a non-zero exit code' do
      subject
      expect($?).to_not be_success
    end
  end

  describe 'list' do
    describe 'push' do
      subject { run("project list push") }

      it { is_expected.to_not include "Illegal Arguments" }

      it 'returns exit code zero' do
        subject
        expect($?).to be_success
      end
    end
  end
end
